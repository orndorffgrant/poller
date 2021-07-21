const assets = [
    {
        url: "https://unpkg.com/htmx.org@1.5.0/dist/htmx.js",
        fileName: "htmx.js",
    },
    {
        url: "https://unpkg.com/alpinejs@2.8.2/dist/alpine.js",
        fileName: "alpine.js",
    },
    {
        url: "https://unpkg.com/charts.css@0.9.0/dist/charts.min.css",
        fileName: "charts.css",
    },
]

for (const asset of assets) {
    console.info(`Fetching ${asset.url}`)
    const assetResponse = await fetch(asset.url);
    if (!assetResponse.ok) {
        throw Error("failed request: " + assetResponse);
    }
    const fileName = `../assets/${asset.fileName}`;
    console.info(`Writing ${fileName}`);
    const buf = await assetResponse.arrayBuffer();
    await Deno.writeFile(fileName, new Uint8Array(buf));
}

export {}