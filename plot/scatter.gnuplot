set terminal png size 1000,1000
set title "Reading and sha-ing n bytes into guest in risc0"
set output "scatter_sha.png"
set xlabel "Input length in bytes"
set ylabel "Duration for run+proof in seconds"
# set decimal locale
# # set decimalsign ","
# set format x "%'.0f"
plot 'sha_scatter_sealed'
