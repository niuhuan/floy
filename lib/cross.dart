import 'dart:io';

import 'package:floy/src/rust/api/init.dart';
import 'package:flutter/services.dart';
import 'package:url_launcher/url_launcher.dart';

Cross cross = Cross._();

class Cross {
  const Cross._();

  static const _channel = MethodChannel("cross");

  Future<String> root() async {
    if (Platform.isAndroid || Platform.isIOS) {
      return await _channel.invokeMethod("root");
    } else if (Platform.isWindows || Platform.isMacOS || Platform.isLinux) {
      return desktopRoot();
    } else {
      throw "没有适配的平台";
    }
  }

  Future saveImageToGallery(String path) async {
    if (Platform.isAndroid || Platform.isIOS) {
      return await _channel.invokeMethod("saveImageToGallery", path);
    }
    throw "没有适配的平台";
  }

  Future<int> androidGetVersion() async {
    if (Platform.isAndroid) {
      return await _channel.invokeMethod("androidGetVersion");
    }
    return 0;
  }

  Future<List<String>> loadAndroidModes() async {
    return List.of(await _channel.invokeMethod("androidGetModes"))
        .map((e) => "$e")
        .toList();
  }

  Future setAndroidMode(String androidDisplayMode) {
    return _channel
        .invokeMethod("androidSetMode", {"mode": androidDisplayMode});
  }

  Future androidAppInfo() {
    return _channel.invokeMethod("androidAppInfo", "");
  }
}

/// 打开web页面
Future<dynamic> openUrl(String url) async {
  if (await canLaunch(url)) {
    await launch(
      url,
      forceSafariVC: false,
    );
  }
}
