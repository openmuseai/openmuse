const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
module.exports = (env, argv) => {
    const mobile = Boolean(env && env.mobile);
    const config = require(mobile ? "./webpack.mobile.js" : "./webpack.desktop.js")(env, argv);
    config.entry = {resource: "./src/openMuseResource.ts"};
    const surface = mobile ? "resource-mobile" : "resource";
    config.output.path = path.resolve(__dirname, "stage/build/" + surface);
    config.output.publicPath = "/stage/build/" + surface + "/";
    config.plugins = config.plugins.filter(plugin => !(plugin instanceof HtmlWebpackPlugin));
    const template = mobile ? "src/assets/template/mobile/openMuseResource.tpl" : "src/assets/template/desktop/openMuseResource.tpl";
    config.plugins.push(new HtmlWebpackPlugin({inject: "head", filename: "index.html", template}));
    for (const rule of config.module.rules) {
        if (String(rule.test).includes("tpl")) rule.include.push(path.resolve(__dirname, template));
    }
    return config;
};
