/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".bootstrap.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"../pkg/florest_bg.wasm": function() {
/******/ 			return {
/******/ 				"./florest_bg.js": {
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_boolean_get": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbindgen_boolean_get"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_number_get": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbindgen_number_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_WebGl2RenderingContext_56ad96bfac3f5531": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_instanceof_WebGl2RenderingContext_56ad96bfac3f5531"](p0i32);
/******/ 					},
/******/ 					"__wbg_bindFramebuffer_48c4bf8ff82bf7e9": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_bindFramebuffer_48c4bf8ff82bf7e9"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_clear_4ce66c813d66e77d": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_clear_4ce66c813d66e77d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_clearColor_71f96fd72a7646a6": function(p0i32,p1f32,p2f32,p3f32,p4f32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_clearColor_71f96fd72a7646a6"](p0i32,p1f32,p2f32,p3f32,p4f32);
/******/ 					},
/******/ 					"__wbg_clearDepth_998e0b481aed1b74": function(p0i32,p1f32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_clearDepth_998e0b481aed1b74"](p0i32,p1f32);
/******/ 					},
/******/ 					"__wbg_clearStencil_6ed2ef92289e2523": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_clearStencil_6ed2ef92289e2523"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteFramebuffer_5f58ccb548438c57": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_deleteFramebuffer_5f58ccb548438c57"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteRenderbuffer_6372146d4689793e": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_deleteRenderbuffer_6372146d4689793e"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_depthMask_d79eec8e156a2cfb": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_depthMask_d79eec8e156a2cfb"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_disable_b05e075ae54fa448": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_disable_b05e075ae54fa448"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_enable_766e546395da5a5d": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_enable_766e546395da5a5d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getExtension_73bff3c015bacd4a": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_getExtension_73bff3c015bacd4a"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getParameter_e3aea13dd0a2904d": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_getParameter_e3aea13dd0a2904d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_isEnabled_051836951c2374ba": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_isEnabled_051836951c2374ba"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_scissor_5802aaee71f2eb0e": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_scissor_5802aaee71f2eb0e"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_viewport_da0901eee69b9909": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_viewport_da0901eee69b9909"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Window_c4b70662a0d2c5ec": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_instanceof_Window_c4b70662a0d2c5ec"](p0i32);
/******/ 					},
/******/ 					"__wbg_document_1c64944725c0d81d": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_document_1c64944725c0d81d"](p0i32);
/******/ 					},
/******/ 					"__wbg_getElementById_f3e94458ce77f0d0": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_getElementById_f3e94458ce77f0d0"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlCanvasElement_25d964a0dde6717e": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_instanceof_HtmlCanvasElement_25d964a0dde6717e"](p0i32);
/******/ 					},
/******/ 					"__wbg_width_555f63ab09ba7d3f": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_width_555f63ab09ba7d3f"](p0i32);
/******/ 					},
/******/ 					"__wbg_height_7153faec70fbaf7b": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_height_7153faec70fbaf7b"](p0i32);
/******/ 					},
/******/ 					"__wbg_getContext_f701d0231ae22393": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_getContext_f701d0231ae22393"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_be86524d73f67598": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_newnoargs_be86524d73f67598"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_888d259a5fefc347": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_call_888d259a5fefc347"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_is_0f5efc7977a2c50b": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_is_0f5efc7977a2c50b"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_self_c6fbdfc2918d5e58": function() {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_self_c6fbdfc2918d5e58"]();
/******/ 					},
/******/ 					"__wbg_window_baec038b5ab35c54": function() {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_window_baec038b5ab35c54"]();
/******/ 					},
/******/ 					"__wbg_globalThis_3f735a5746d41fbd": function() {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_globalThis_3f735a5746d41fbd"]();
/******/ 					},
/******/ 					"__wbg_global_1bc0b39582740e95": function() {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_global_1bc0b39582740e95"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbg_buffer_397eaa4d72ee94dd": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_buffer_397eaa4d72ee94dd"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_69ece9a32977b4d4": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_new_69ece9a32977b4d4"](p0i32);
/******/ 					},
/******/ 					"__wbg_set_7b1876d7b71366be": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_set_7b1876d7b71366be"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_length_efebca886dbee16f": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_length_efebca886dbee16f"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_9e075f196d08441f": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_new_9e075f196d08441f"](p0i32);
/******/ 					},
/******/ 					"__wbg_set_67ac80f2b2b25cc1": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_set_67ac80f2b2b25cc1"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_length_1d0477eab3803c38": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_length_1d0477eab3803c38"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_8b45a9becdb89691": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_new_8b45a9becdb89691"](p0i32);
/******/ 					},
/******/ 					"__wbg_set_81e73b00bc542aa8": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_set_81e73b00bc542aa8"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_length_1ba55d1192582e82": function(p0i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbg_length_1ba55d1192582e82"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_memory": function() {
/******/ 						return installedModules["../pkg/florest_bg.js"].exports["__wbindgen_memory"]();
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["../pkg/florest_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/florest_bg.wasm":"38651376b94bf17fe7cd"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./bootstrap.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("__webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./index.js */ \"./index.js\"))\n  .catch(e => console.error(\"Error importing `index.js`:\", e));\n\n//# sourceURL=webpack:///./bootstrap.js?");

/***/ })

/******/ });