set terminal png size 1000,1000
set output "scatter.png"
set xlabel "Input length in bytes"
set ylabel "Duration for run+proof in seconds"
set format "%'.0f"
plot 'bench_with_seal'
