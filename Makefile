run:
	npx concurrently --kill-others "cd api && cargo run" "cd web-app && yarn dev"
