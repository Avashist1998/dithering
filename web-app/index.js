import init, { black_and_white, floyd_steinberg_dithering, atkinson_dithering } from './pkg/web_app.js';


document.getElementById('upload').addEventListener('change', async (event) => {
    const file = event.target.files[0];
    const reader = new FileReader();

    reader.onload = async function(e) {
        const img = new Image();
        img.src = e.target.result;

        img.onload = async function() {
            const canvas = document.getElementById('input-canvas');
            const ctx = canvas.getContext('2d');
            canvas.width = img.width;
            canvas.height = img.height;
            ctx.drawImage(img, 0, 0);

            const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
            const data = imageData.data;
            
            // Initialize the Rust Wasm module
            await init();
            // Process the image data using the Rust Wasm function
            const binaryData = black_and_white(data, canvas.width);
            
            const blackWhiteCanvas = document.getElementById('b-w-canvas');
            const bwCtx = blackWhiteCanvas.getContext('2d');
            
            blackWhiteCanvas.width = img.width;
            blackWhiteCanvas.height = img.height;
            const binaryImageData = new ImageData (new Uint8ClampedArray(binaryData), blackWhiteCanvas.width, blackWhiteCanvas.height);
            bwCtx.putImageData(binaryImageData, 0, 0);
            
            
            const FSData = floyd_steinberg_dithering(data, canvas.width);
            console.log(FSData);
            const FSCanvas = document.getElementById('f-s-canvas');
            const fsCtx = FSCanvas.getContext('2d');
            
            FSCanvas.width = img.width;
            FSCanvas.height = img.height;
            const FSImageData = new ImageData (new Uint8ClampedArray(FSData), FSCanvas.width, FSCanvas.height);
            fsCtx.putImageData(FSImageData, 0, 0);


            const AData = atkinson_dithering(data, canvas.width);
            console.log(AData);
            const ACanvas = document.getElementById('a-canvas');
            const ACtx = ACanvas.getContext('2d');
            
            ACanvas.width = img.width;
            ACanvas.height = img.height;
            const AImageData = new ImageData (new Uint8ClampedArray(AData), ACanvas.width, ACanvas.height);
            ACtx.putImageData(AImageData, 0, 0);

            const elements = document.querySelectorAll('.image-matrix');
            elements.forEach(element => {
                element.style.visibility = "visible";
            })
        };
    };

    reader.readAsDataURL(file);
});
