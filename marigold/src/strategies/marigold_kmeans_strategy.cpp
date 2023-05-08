#include "../interfaces/interface_kmeans.hpp"
#include "../kmeans_utils/utils.cpp"
#include <cstring>
#include <algorithm>
#include <chrono>


class MARIGOLDKmeansStrategy : public KmeansStrategy {
    public:
        int* run(Dataset* data, double* final_centroids, int* final_iter, double* final_inertia) {
            std::chrono::high_resolution_clock::time_point t1;
            std::chrono::high_resolution_clock::time_point t2;
            std::chrono::high_resolution_clock::time_point t1_1;
            t1 = std::chrono::high_resolution_clock::now();
            int iter = 0;
            bool converged = false;

            Calculate_squared_botup(d, n, data_ptr, data_ss, l_pow);
            t1_1 = std::chrono::high_resolution_clock::now();
            for (int i = 0; i < k; i++) {
                for (int j = i; j < k; j++) {
                    c_to_c[i][j] = 0;
                    c_to_c[j][i] = 0;
                }
            }

            while ((iter < max_inter) && (!converged)) {
                //calculate square centroids
                Calculate_squared_botup(d, k, centroids, centroid_ss, l_pow);

                //assign to centroids
                for (int i = 0; i < n; i++) {
                    double val = near_[labels[i]] < l_hamerly[i] ? l_hamerly[i] : near_[labels[i]];
                    if (u_elkan[i] > val) {
                         MG_SetLabel(i);
                    }
                }
                converged = Recalculate(data_ptr, centroids, old_centroids, cluster_count, labels, div, n, k, d, feature_cnt, final_inertia);
                if (!converged) {
                    //TODO: refactor location of .. you know the drill
                    Update_bounds(data_ptr, centroids, c_to_c, centroid_ss, l_elkan, u_elkan, l_hamerly, labels, div, near_, n, k, d, feature_cnt);

                }
                iter++;
            }

            //for (int j = 0; j < k; j++) {
            //    std::cout << cluster_count[j] << " ";
            //}
            //std::cout << std::endl;
            //std::cout << "Iter:" << iter << " Feature_cnt: " << feature_cnt << std::endl;
            //std::cout << "First centroid vals: " << centroids[0] << " " << centroids[1] << " " << centroids[2] << std::endl;

            *final_iter = iter;
            t2 = std::chrono::high_resolution_clock::now();
            std::chrono::duration<double> time_span = std::chrono::duration_cast<std::chrono::duration<double>>(t2 - t1);
            std::chrono::duration<double> time_span2 = std::chrono::duration_cast<std::chrono::duration<double>>(t2 - t1_1);
            //std::cout << time_span.count()*1000 << "  " << time_span2.count()*1000 << std::endl;
            for (int i = 0; i < k; i++) {
                for (int j = 0; j < d; j++) {
                    final_centroids[i*d+j] = centroids[i*d+j];
                }
            }
            //std::cout << "First centroid vals: " << final_centroids[0] << " " << final_centroids[1] << " " << final_centroids[2] << std::endl;
            //std::cout << "First centroid vals: " << centroids[0] << " " << centroids[1] << " " << centroids[2] << std::endl;
            return labels;
        };

        void MG_SetLabel(const int x) {
            int l = 0;
            int *mask = new int[k];
            std::fill_n(mask, k, 1);

            double *dist = new double[k];
            for (int j = 0; j < k; j++) {
                dist[j] = data_ss[x][0]+centroid_ss[j][0];
            }


            double val;
            double UB, LB;

            int mask_sum = k;

            while (l <= L && mask_sum > 1) {
                for (int j = 0; j < k; j++) {
                    if (mask[j] != 1) continue;

                    //Elkan prune
                    val = std::max(l_elkan[x][j], 0.5 * c_to_c[labels[x]][j]);
                    if (u_elkan[x] < val) {     //Elkan check
                        mask[j] = 0;            //Mark as pruned centroid
                    } else {
                        //DistToLevel params (int x, int c, int d, double data[], double centroids[], double* data_ss[], double* centroid_ss[], double* dots[], int l, int L)
                        DistToLevel_bot(x, j, d, data_ptr, centroids, data_ss, centroid_ss, l, L, dist[j], UB, LB, feature_cnt, l_pow);
                        LB = sqrt(std::max(0.0, LB));
                        //if (LB > l_elkan[x][j]) {

                            if (LB > l_elkan[x][j]) {
                                l_elkan[x][j] = LB; //Keep maximum LB per c
                            }
                        //}

                        UB = sqrt(std::max(0.0, UB));
                        if (UB < u_elkan[x]) {
                            labels[x] = j;
                            u_elkan[x] = UB; //Keep minimum UB across c
                        }
                    }
                }
                mask_sum = 0;
                for (int j = 0; j < k; j++) {
                    mask_sum += mask[j];
                }
                l++;
            }

            //delete[] mask;
            //delete[] dist;
            //END: Updated labels, l_elkan[x][.], u_elkan[x]
        }

        //void clear() {}
        void clear() {
            for (int i = 0; i < n; i++) {
                delete[] l_elkan[i];
            }
            delete[] l_elkan;

            delete[] l_hamerly;

            delete[] u_elkan;

            delete[] near_;

            delete[] div;


            for (int i = 0; i < k; i++) {
                delete[] c_to_c[i];
            }
            delete[] c_to_c;




            for (int i = 0; i < n; i++) {
                delete[] data_ss[i];
            }
            delete[] data_ss;

            for (int i = 0; i < k; i++) {
                delete[] centroid_ss[i];
            }
            delete[] centroid_ss;

            delete[] labels;

            delete[] cluster_count;


            //delete[] centroids;
            delete[] old_centroids;

            delete[] l_pow;
        }

        void init(int _max_iter, int _n, int _d, int _k, Dataset* _data, double* _centroids_ptr) {

            max_inter = _max_iter;
            n = _n;
            d = _d;
            k = _k;
            data_ptr = _data->get_data_pointer();
            feature_cnt = 0;

            //stepwise levels
            L = ceil(log10(d)/log10(4));

            //bounds
            l_elkan = new double*[n];
            for (int i = 0; i < n; i++) {
                l_elkan[i] = new double[k];
                std::fill(l_elkan[i], l_elkan[i]+k, 0.0);
            }

            l_hamerly = new double[n];
            std::fill(l_hamerly, l_hamerly+n, 0);


            u_elkan = new double[n];
            std::fill(u_elkan, u_elkan+n, std::numeric_limits<double>::max());


            near_ = new double[k];
            std::fill(near_, near_+k, 0);

            div = new double[k];

            //c_to_c
            c_to_c = new double*[k];//[new double[k]];
            for (int i = 0; i < k; i++) {
                c_to_c[i] = new double[k];
            }

            l_pow = new int[L+1];
            for (int i = 0; i <= L; i++) {
                if (i == L && log10(d)/log10(4) < L) {
                    l_pow[i] = sqrt(d);
                } else {
                    l_pow[i] = int(pow(2,i));
                }
            }


            //squared
            data_ss = new double*[n];
            for (int i = 0; i < n; i++) {
                data_ss[i] = new double[L+2];
            }

            centroid_ss = new double*[k];
            for (int i = 0; i < k; i++) {
                centroid_ss[i] = new double[L+2];
            }

            //Init labels
            labels = new int[n];
            std::fill(labels, labels+n, 0);

            //Init cluster_counts
            cluster_count = new double[k];

            //Init centroids
            centroids = _centroids_ptr;//new double[k*d];
            old_centroids = new double[k*d];

            //memcpy(centroids, data_ptr , sizeof(double)*k*d); //Initial dentroids

        }
    private:

        int max_inter;
        int n;
        int d;
        int k;
        int L;
        double* centroids;
        double* old_centroids;
        double* cluster_count;
        long long feature_cnt;

        //double** dots;

        double** l_elkan;
        double* l_hamerly;
        double* u_elkan;


        int* l_pow;

        double* near_;

        double* div;

        double** c_to_c;
        double** data_ss;
        double** centroid_ss;

        //x to c [x*k+c]
        int* labels;

        double* data_ptr;// = data->get_data_pointer();

};
