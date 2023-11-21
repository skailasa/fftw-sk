bindgen \
    --use-core \
    --whitelist-type="^fftw.*" \
    --whitelist-var="^FFTW.*" \
    --default-enum-style=rust \
    wrapper.h \
    > src/fftw.rs
