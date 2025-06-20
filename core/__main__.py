import uvicorn

import api
import analysis

def main():
    uvicorn.run(api.app, port=11451)

if __name__ == "__main__":
    main()