cargo rustc -- --emit=obj
sudo ld -x --shared -o /lib/x86_64-linux-gnu/security/mypam.so target/debug/libpam_sol.so 
# https://github.com/beatgammit/simple-pam