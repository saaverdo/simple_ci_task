### Sample CI task

Build CI pipeline for a rust "hello-world" service.  

- CI pipeline should run linters for docker files or/and code.  
- CI pipeline should build container image, tag it and push to docker image registry.  

---
Service uses port `8080`  
If it run correctly, `curl http://localhost:8080` shall return `Hello world!`  

There can be mistakes in code. Correct them. ;)
