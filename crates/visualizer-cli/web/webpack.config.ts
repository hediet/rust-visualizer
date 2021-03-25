import * as webpack from "webpack";
import path = require("path");
import HtmlWebpackPlugin = require("html-webpack-plugin");
import ZipPlugin = require("zip-webpack-plugin");

const r = (file: string) => path.resolve(__dirname, file);

module.exports = {
	entry: r("src/index.tsx"),
	output: { path: r("dist") },
	resolve: {
		extensions: [".webpack.js", ".web.js", ".ts", ".tsx", ".js"],
	},
	devtool: "source-map",
	module: {
		rules: [
			{
				test: /\.less$/,
				use: ["style-loader", "css-loader", "less-loader"],
			},
			{ test: /\.css$/, use: ["style-loader", "css-loader"] },
			{
				test: /\.scss$/,
				use: ["style-loader", "css-loader", "sass-loader"],
			},
			{
				test: /\.(jpe?g|png|gif|eot|ttf|svg|woff|woff2|md)$/i,
				loader: "file-loader",
			},
			{
				test: /\.tsx?$/,
				loader: "ts-loader",
				options: { transpileOnly: true },
			},
			{ test: /\.(png|gif|cur|jpg)$/, loader: "url-loader" },
		],
	},
	plugins: [
		new HtmlWebpackPlugin({
			templateContent: `
		<!DOCTYPE html>
		<html>
			<head>
			<meta charset="utf-8">
			<title>Debug Visualizer</title>
			</head>
			<body>
			</body>
		</html>`,
		}),
		new ZipPlugin({
			filename: "bundle.zip",
			exclude: [/\.map$/],
		}),
	],
} as webpack.Configuration;
