# Changelog

## [v0.1.0] - 2026-02-24

### Added
- Initial project scaffold: Rust + axum 0.8 + askama 0.12 + HTMX 2.x
- mkube API client: pods, nodes, events, consistency, networks, images
- microdns client: zones, records CRUD across all 4 networks (gt, g10, g11, gw)
- Registry v2 client: catalog, tags, manifests, image configs with usage tracking
- Dashboard overview page with pod counts, consistency, DNS status, recent events (auto-refresh 10s)
- Pods page: list with filtering, detail with containers/volumes/annotations, restart/delete actions
- Log viewer: streaming logs per container with HTMX polling (2s auto-refresh)
- DNS management: zone list from all networks, full record CRUD (A, AAAA, CNAME, PTR, NS, TXT)
- Networks page: list and detail with CIDR, gateway, IPAM ranges
- Nodes page: list and detail with conditions, capacity
- Registry browser: repos with tag counts, storage usage, in-use status, image details
- Events page: timeline with namespace filtering
- Health endpoint for liveness probes
- Dark theme CSS with sidebar navigation
- Build scripts for ARM64 musl cross-compile + scratch container
- Pod spec for deployment on gt network
