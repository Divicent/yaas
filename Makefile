deploy-api:
	cd api 
	git checkout main 
	git pull --force 
	docker-compose up --force-recreate --build -d
	docker image prune -f
