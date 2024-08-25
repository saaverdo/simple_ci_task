### Sample CI task

Build CI process for a rust hello-world service.  

- CI pipeline should run linters for docker files or/and code.  
- CI pipeline should build container image, tag it and push to docker image registry.  
- Image should be checked to run correctly.  

---
Service uses port `8080`  
If it run correctly, `curl http://localhost:8080` will return `Hello world!`  

There can be mistakes in code. Correct them.  
