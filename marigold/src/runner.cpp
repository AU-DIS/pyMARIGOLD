#ifdef __WIN32__
#include <windows.h>
#include <winbase.h>
#ifdef BUILD_LIB
#define LIB_FUNC __declspec(dllexport)
#else
#define LIB_FUNC __declspec(dllimport)
#endif
#else
#define LIB_FUNC       // Blank
#endif

#include "kmeans_runner.cpp"
#include "strategies/marigold_kmeans_strategy.cpp"

KmeansRunner runner(std::make_unique<MARIGOLDKmeansStrategy>());

/*extern "C" LIB_FUNC int* run(const char* file_name, int n, int d, int k, double* init_centroids, double* final_centroids, int* final_iter) {
    runner.init_run(n,d,k,file_name);
    return runner.run_wo_clear_kmeans(n,d,k, init_centroids, final_centroids, final_iter);
}*/

extern "C" LIB_FUNC int* run(double* data_pnt, int n, int d, int k, double* init_centroids, double* final_centroids, int* final_iter, double* final_inertia) {
    runner.init_run(n,d,k,data_pnt);
    return runner.run_wo_clear_kmeans(n,d,k, init_centroids, final_centroids, final_iter, final_inertia);
}

extern "C" LIB_FUNC void clear() {
    runner.clear();
}
