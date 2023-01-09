```bash
CXX=clang++ CC=clang cargo build
```

I also overwrite $(which cc) with $(which clang) and the same for $(which c++) and $(which clang++); not sure that was a good idea?
