# Frontend setup

## Environment setup

1. Install node-version-manager & node

Can be found at this [site](https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating) and can be installed using

```sh
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash
```

This edits your login script ('.bashrc' or whatever) to add some environment variables. Either add those by hand or (easier) restart/open a new terminal. Then you can install the latest version of node

```sh
nvm install node
```

2. Create a new Vite + Vue + Tailwind.css project

Create the project - install the dev dependencies and initialize the tailwind project

```sh
npm create vite@latest <project-name> -- --template vue
cd <project-name>
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

Edit the tailwind config

```js
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

Edit the styles.css
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

add our `coloris` color chooser dependency

```sh
npm install @melloware/coloris
```

3. Add python as a testing server

```sh
python -m venv .venv
source .venv/bin/activate
python -m pip install --upgrade pip
pip install "fastapi[standard]"
```

create a new test.py file
```py
from fastapi import FastAPI
from fastapi.responses import HTMLResponse, ORJSONResponse
from fastapi.staticfiles import StaticFiles

app = FastAPI()

app.mount("/assets", StaticFiles(directory="dist/assets"), name="assets")

@app.get("/", response_class=HTMLResponse)
async def root():
    with open("dist/index.html", "r") as f:
        return f.read()

@app.get("/foo", response_class=ORJSONResponse)
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
```

add paths to that as you need to test back-end to front-end features

3. Start work on the front-end

Add `.vue` files to the components

Large slider toggle for ON-OFF

```vue
<script setup>
    defineProps({
        left_text: String,
        right_text: String,
    })
</script>

<template>
    <label class="relative flex justify-center items-center p-2 text-xl py-4 font-bold">
        <h2 class="text-right px-4">{{ left_text }}</h2>
        <input type="checkbox" class="absolute left-1/2 -translate-x-1/2 w-full h-full peer appearance-none rounded-md" />
        <span class="w-32 h-10 flex grow-0 items-center ml-4 p-1 bg-slate-800 rounded-lg duration-300 ease-in-out peer-checked:bg-cyan-700 after:w-20 after:h-8 after:bg-slate-400 after:rounded-lg after:shadow-md after:duration-300 peer-checked:after:bg-slate-100 peer-checked:after:translate-x-10"></span>
        <h2 class="text-left px-4">{{ right_text }}</h2>
    </label>
</template>
```

Range sliders for parameters

```vue
<script setup>
    defineProps({
        name: String,
        min: Number,
        max: Number,
    })
</script>

<template>
    <div class="p-2 text-xl py-4 font-bold">
        <label for="range-{{ name }}" class="block mb-2 text-left text-gray-900 dark:text-white">{{ name }}</label>
        <input 
            id="range-{{ name }}" 
            type="range" 
            min="{{ min }}" 
            max="{{ max }}" 
            value="{{ (min+max)/2 }}" 
            class="range accent-cyan-700 w-full h-2 bg-slate-200 rounded-lg appearance-none cursor-pointer dark:bg-slate-700"
        />
    </div>
</template>```

Then import them and use them in the main 'App.vue'

```vue
<script setup>
import toggle from './components/toggle.vue'
import slider from './components/slider.vue'
</script>

<template>
  <h1 class="text-3xl font-bold underline">
    Hello, world!
  </h1>
  <toggle left_text="Off" right_text="On"/>
  <slider name="foo" min="0" max="100" />
  <slider name="bar" min="0" max="100" />
</template>
```

4. Build and serve the app

Build the app

```sh
npm run build
```

This creates an index.html and set of assets in the 'dist' folder - these can be served with our fast-api server that will
also act as a stub for when we need to test front-end/back-end functionality

```sh
fastapi dev app.py
```
