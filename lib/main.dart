import 'package:floy/screens/init_screen.dart';
import 'package:flutter/material.dart';
import 'package:floy/src/rust/api/init.dart';
import 'package:floy/src/rust/frb_generated.dart';

import 'commons/router.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      navigatorObservers: [
        routeObserver,
      ],
      home: const InitScreen(),
    );
  }
}
