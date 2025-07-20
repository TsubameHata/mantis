import uvicorn

import api

def main():
    uvicorn.run(api.app, port=11451, log_config=None)

if __name__ == "__main__":
    main()