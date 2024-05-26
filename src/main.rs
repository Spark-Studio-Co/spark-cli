use clap::{Command, Arg};
use std::fs;
use std::env;
use std::process::Command as SystemCommand;
use std::path::{Path, PathBuf};


fn main() {
    println!("Hello, world!");

    // Initializing new project
    let matches = Command::new("Spark CLI")
        .version("1.0")
        .author("Spark Studio")
        .about("Helps initialize projects with different architectures")
        .arg(Arg::new("cli_commands")
            .help("Specifies the type of project to initialize")
            .required(true)
            .index(1))
        .arg(Arg::new("project_name")
            .help("Name of the project directory")
            .required(true)
            .index(2))
        .get_matches();

    let project_type = matches.get_one::<String>("cli_commands").expect("required and checked by clap");
    let project_name = matches.get_one::<String>("project_name").expect("required and checked by clap");


    match project_type.as_str() { // Use as_str() to convert &String to &str
        "react@fsd" => init_react_fsd(project_name),
        _ => println!("Project type not supported"),
    }

    fn init_react_fsd(project_name: &str) {
        // Determine the current working directory
        let cwd = env::current_dir().expect("Failed to determine the current directory");
        println!("Current directory: {:?}", cwd);

        // Set up the full path for the new project
        let full_project_path = cwd.join(project_name);

        // Create the base path for the project
        if !full_project_path.exists() {
            fs::create_dir_all(&full_project_path).unwrap();
        }

        let paths = [
            "src/app",
            "src/assets",
            "src/widgets",
            "src/features",
            "src/entities",
            "src/shared",
            "src/redux",
            "public"
        ];

        // Create directories
        for path in paths.iter() {
            let full_path = full_project_path.join(path);
            fs::create_dir_all(&full_path).unwrap_or_else(|_| panic!("Failed to create directory: {:?}", full_path));
        }

        let package_json_path = full_project_path.join("package.json");
        let package_json_content = r#"
    {
      "name": "my-react-app",
      "version": "0.0.1",
      "scripts": {
        "dev": "vite",
        "build": "vite build",
        "serve": "vite preview"
      },
      "dependencies": {
        "react": "^18",
        "react-dom": "^18",
        "breakpoint-slicer": "^3.0.0-beta.1",
        "gsap": "^3.12.5"
      },
      "devDependencies": {
        "@vitejs/plugin-react": "^1.0.0",
        "vite": "^2.0.0",
        "tailwindcss": "^3.3.0",
        "typescript": "5.3.3",
        "sass": "^1.70.0",
        "autoprefixer": "^10.0.1",
        "eslint": "^8",
        "eslint-config-next": "14.1.0",
        "postcss": "^8",
        "prettier": "^3.2.5"
      }
    }
    "#;

        fs::write(&package_json_path, package_json_content).unwrap_or_else(|_| panic!("Failed to write package.json"));

        // Create vite.config.js
        let vite_config_path = full_project_path.join("vite.config.js");
        let vite_config_content = r#"
    import { defineConfig } from 'vite'
    import react from '@vitejs/plugin-react'
    import path from "path";

    export default defineConfig({
      plugins: [react()],
      resolve: {
        alias: {
            "@shared": path.resolve(__dirname, "src/shared"),
            "@widgets": path.resolve(__dirname, "src/widgets"),
            "@entities": path.resolve(__dirname, "src/entities"),
            "@features": path.resolve(__dirname, "src/features"),
            "@pages": path.resolve(__dirname, "src/pages"),
            "@assets": path.resolve(__dirname, "src/assets"),
    },
  },
    })
    "#;

        fs::write(&vite_config_path, vite_config_content).unwrap_or_else(|_| panic!("Failed to write vite.config.js"));


        let ts_config_path: PathBuf = full_project_path.join("tsconfig.json");
        let tsconfig_content = r#"
      {
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "baseUrl": "./",
    "paths": {
      "@shared/*": ["src/shared/*"],
      "@widgets/*": ["src/widgets/*"],
      "@entities/*": ["src/entities/*"],
      "@features/*": ["src/features/*"],
      "@pages/*": ["src/pages/*"],
      "@assets/*": ["src/assets/*"]
    },

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}

"#;

        fs::write(&ts_config_path, tsconfig_content).unwrap_or_else(|_| panic!("Failed to write tsconfig.json"));

        // Create tsconfig.node.json
        let tsconfig_node_path = full_project_path.join("tsconfig.node.json");
        let tsconfig_node_content = r#"
        {
            "compilerOptions": {
              "target": "ES2020",
              "useDefineForClassFields": true,
              "lib": ["ES2020", "DOM", "DOM.Iterable"],
              "module": "ESNext",
              "skipLibCheck": true,

              /* Bundler mode */
              "moduleResolution": "bundler",
              "allowImportingTsExtensions": true,
              "resolveJsonModule": true,
              "isolatedModules": true,
              "noEmit": true,
              "jsx": "react-jsx",

              /* Linting */
              "strict": true,
              "noUnusedLocals": true,
              "noUnusedParameters": true,
              "noFallthroughCasesInSwitch": true
            },
            "include": ["src"],
            "references": [{ "path": "./tsconfig.node.json" }]
}
"#;

        fs::write(&tsconfig_node_path, tsconfig_node_content).unwrap_or_else(|_| panic!("Failed to write tsconfig.node.json"));


        // Path for .gitignore file
        let gitignore_path = full_project_path.join(".gitignore");
        let gitignore_content = r#"
        # Logs
        logs
        *.log
        npm-debug.log*
        yarn-debug.log*
        yarn-error.log*
        pnpm-debug.log*
        lerna-debug.log*

        node_modules
        dist
        dist-ssr
        *.local

        # Editor directories and files
        .vscode/*
        !.vscode/extensions.json
        .idea
        .DS_Store
        *.suo
        *.ntvs*
        *.njsproj
        *.sln
        *.sw?
"#;

        fs::write(&gitignore_path, gitignore_content).unwrap_or_else(|_| panic!("Failed to write .gitignore"));


        // Path for .eslintrc.cjs
        let eslintrc_path = full_project_path.join(".eslintrc.cjs");
        let eslintrc_content = r#"
        module.exports = {
            root: true,
            env: { browser: true, es2020: true },
            extends: [
              'eslint:recommended',
              'plugin:@typescript-eslint/recommended',
              'plugin:react-hooks/recommended',
            ],
            ignorePatterns: ['dist', '.eslintrc.cjs'],
            parser: '@typescript-eslint/parser',
            plugins: ['react-refresh'],
            rules: {
              'react-refresh/only-export-components': [
                'warn',
                { allowConstantExport: true },
              ],
            },
}
"#;
        fs::write(&eslintrc_path, eslintrc_content).unwrap_or_else(|_| panic!("Failed to write eslintrc.cjs"));


        // Path for the main.jsx file
        let main_tsx_path = full_project_path.join("src/main.tsx");


        // Simulated content of main.jsx as an example
        let main_tsx_content = r#"
    import React from 'react'
    import ReactDOM from 'react-dom'
    import App from './App'

    ReactDOM.render(
      <React.StrictMode>
        <App />
      </React.StrictMode>,
      document.getElementById('root')
    )
    "#;

        // Write main.tsx file
        fs::write(&main_tsx_path, main_tsx_content).unwrap_or_else(|_| panic!("Failed to write main.jsx"));

        // Create tailwind config
        let  tailwind_config_path = full_project_path.join("tailwind.config.js");
        let tailwind_config_content = r#"
        /** @type {import('tailwindcss').Config} */
        export default {
            content: [
                "./pages/**/*.{js,ts,jsx,tsx,mdx}",
                "./components/**/*.{js,ts,jsx,tsx,mdx}",
                "./app/**/*.{js,ts,jsx,tsx,mdx}",
                "./src/**/*.{js,jsx,ts,tsx}",
            ],
            theme: {
                extend: {
                    colors: {
                    },
                },
            },
            plugins: [],
        };
"#;

        fs::write(&tailwind_config_path, tailwind_config_content).unwrap_or_else(|_| panic!("Failed to write tailwind.config.js"));


        // Create postcss.config.cjs
        let postcss_config_path = full_project_path.join("postcss.config.cjs");
        let postcss_content  = r#"
        module.exports = {
            plugins: {
              tailwindcss: {},
            },
        };
"#;

        fs::write(&postcss_config_path, postcss_content).unwrap_or_else(|_| panic!("Failed to write postcss.cjs"));


        // Create vite.env.d.ts
        let vite_env_path = full_project_path.join("src/vite-env.d.ts");
        let vite_env_content = r#"
        /// <reference types="vite/client" />
        "#;

        fs::write(&vite_env_path, vite_env_content).unwrap_or_else(|_| panic!("Failed to write vite-env.d.ts"));


        // Create App.jsx
        let app_tsx_path = full_project_path.join("src/App.tsx");
        let app_tsx_content = r#"

        import "@shared/styles/global.scss"

        export const App() {
          return (
            <div>
              <h1>Hello from Feature-Sliced Design!</h1>
            </div>
          )
        }

        export default App
    "#;
        fs::write(&app_tsx_path, app_tsx_content).unwrap_or_else(|_| panic!("Failed to write App.jsx"));


        println!("React project initialized with Feature-Sliced Design at {}", full_project_path.display());

// Create index.html
        let index_html_path = full_project_path.join("public/index.html");
        let index_html_content = r#"
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>My React App</title>
      </head>
      <body>
        <div id="root"></div>
        <script type="module" src="/src/main.jsx"></script>
      </body>
    </html>
    "#;
        fs::write(&index_html_path, index_html_content).unwrap_or_else(|_| panic!("Failed to write index.html"));

        println!("React project initialized with Feature-Sliced Design at {}", full_project_path.display());

        install_dependencies(&full_project_path);
    }

    // Download all dependencies for this project
    fn install_dependencies(project_path: &Path) {
        println!("Installing dependencies...");

        let status = SystemCommand::new("npm")
            .arg("install")
            .current_dir(project_path)
            .status()
            .expect("Failed to run npm install");

        if status.success() {
            println!("Dependencies installed successfully.");
        } else {
            println!("Failed to install dependencies.");
        }
    }
}