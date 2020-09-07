#!/usr/bin/env node
"use strict";
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
var cac_1 = __importDefault(require("cac"));
var fs_1 = __importDefault(require("fs"));
var path_1 = __importDefault(require("path"));
var langs_1 = __importDefault(require("./langs"));
var utils = __importStar(require("./utils"));
// tslint:disable-next-line: no-var-requires
var build = require("tailwindcss/lib/cli/commands/build");
var loadAdapters = function () {
    return langs_1.default.reduce(function (agg, lang) {
        var _a;
        return (__assign(__assign({}, agg), (_a = {}, _a[lang] = Promise.resolve().then(function () { return __importStar(require("./adapters/" + lang)); }), _a)));
    }, 
    // tslint:disable-next-line: no-object-literal-type-assertion
    {});
};
var main = function (_a) {
    var config = _a.config, cssInput = _a.cssInput, cssOutput = _a.cssOutput, lang = _a.lang, output = _a.output;
    return __awaiter(void 0, void 0, void 0, function () {
        var adapters, adapter;
        return __generator(this, function (_b) {
            switch (_b.label) {
                case 0: return [4 /*yield*/, loadAdapters()];
                case 1:
                    adapters = _b.sent();
                    if (!utils.isValidLang(lang)) {
                        // tslint:disable-next-line: no-console
                        console.error("lang should be one of " + Object.keys(adapters).join(", ") + ", got " + lang);
                        return [2 /*return*/, process.exit(1)];
                    }
                    return [4 /*yield*/, adapters[lang]];
                case 2:
                    adapter = _b.sent();
                    return [4 /*yield*/, utils.shutDownLog(function () { return __awaiter(void 0, void 0, void 0, function () {
                            return __generator(this, function (_a) {
                                switch (_a.label) {
                                    case 0: return [4 /*yield*/, build.run([cssInput], {
                                            config: config ? [config] : [],
                                            output: [cssOutput],
                                        })];
                                    case 1:
                                        _a.sent();
                                        try {
                                            fs_1.default.mkdirSync(output, { recursive: true });
                                        }
                                        catch (error) {
                                            // tslint:disable-next-line: no-console
                                            console.error("Couldn't create directory " + output);
                                            return [2 /*return*/, process.exit(1)];
                                        }
                                        adapter.save(output, utils.readClasses(cssOutput));
                                        return [2 /*return*/];
                                }
                            });
                        }); })];
                case 3:
                    _b.sent();
                    // tslint:disable-next-line: no-console
                    console.log("Successfully generated files!");
                    return [2 /*return*/];
            }
        });
    });
};
var cli = cac_1.default();
// CLI options
cli.command("", "Generates code and css from a tailwind config file");
cli.option("-c, --config <config>", "Provide tailwind.config.js path");
cli.option("-l, --lang <lang>", "Language used in generated code (" + langs_1.default.join("|") + ")");
cli.option("-o, --output <dir>", "Provide directory for generated code", {
    default: "./src",
});
cli.option("--cssOutput <stylesheet>", "Provide full path (including file name) for generated css stylesheet", {
    default: "./tailwind.css",
});
cli.option("--cssInput <stylesheet>", "Provide path of your css stylesheet which uses the @tailwind directive to inject Tailwind's preflight and utilities styles into your CSS", { default: path_1.default.join(__dirname, "..", "assets", "input.css") });
cli.help();
// Run the command with options
main(cli.parse().options);
