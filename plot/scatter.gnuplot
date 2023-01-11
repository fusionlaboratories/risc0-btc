set terminal png size 1000,1000
set title "Reading and summing n bytes into guest in risc0"
set output "scatter_sum.png"
set xlabel "Input length in bytes"
set ylabel "Duration for run+proof in seconds"
# set decimal locale
# # set decimalsign ","
# set format x "%'.0f"
plot 'sum_with_seal'
