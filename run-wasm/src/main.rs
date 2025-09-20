fn main() {
    cargo_run_wasm::run_wasm_cli_with_css(
        r#"
    body {
      justify-content: center;
      color: white;
      font-family: arial;
      background-color: #171414;

      }
      
    canvas {
      padding: 0;
      margin: auto;
      border: 1px solid #5ea1b0;
      margin-bottom: 20px;
      display: block;
    }
        
    header {
      margin-bottom: 20px;
      font-size: 40px;
      
      width: 100%;
      text-align: center;
    }
      
    div.display {
    }

    h4 {
      text-align: center;
    }

    footer {
      display: block;
      position: fixed;
      bottom: 10px;
      left: 10px;
    }

    a {
      color: gainsboro;
    }
    "#,
    );
}
