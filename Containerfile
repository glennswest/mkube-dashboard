FROM scratch
COPY target/aarch64-unknown-linux-musl/release/mkube-dashboard /dashboard
COPY static/ /static/
EXPOSE 8080
ENTRYPOINT ["/dashboard", "--config", "/etc/dashboard/config.yaml"]
