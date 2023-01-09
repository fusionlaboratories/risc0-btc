set terminal png size 1000,1000
set title "Reading n bytes into guest in risc0"
set output "scatter.png"
set xlabel "Input length in bytes"
set ylabel "Duration for run+proof in seconds"
# set decimal locale
# # set decimalsign ","
# set format x "%'.0f"
plot 'bench_with_seal'
