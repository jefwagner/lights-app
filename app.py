from fastapi import FastAPI
from fastapi.responses import HTMLResponse, ORJSONResponse
from fastapi.staticfiles import StaticFiles

app = FastAPI()

app.mount("/assets", StaticFiles(directory="frontend/dist/assets"), name="assets")

@app.get("/", response_class=HTMLResponse)
async def root():
    with open("frontend/dist/index.html", "r") as f:
        return f.read()

@app.get("/params", response_class=ORJSONResponse)
async def params():
    return ORJSONResponse(
        [
            {
                "name": "foo",
                "type": "slider",
                "value": 50,
                "meta": {"min": 0, "max": 100}
            },
            {
                "name": "bar",
                "type": "toggle",
                "value": False,
                "meta": None
            },
            {
                "name": "baz",
                "type": "color",
                "value": "#646cff",
                "meta": None
            },
        ]
    )
