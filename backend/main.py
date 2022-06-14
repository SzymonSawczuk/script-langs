from weakref import proxy
import uvicorn

if __name__ == "__main__":
    uvicorn.run("app.api:app", host="0.0.0.0", port=80, proxy_headers=True)
    