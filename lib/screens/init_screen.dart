import 'package:floy/cross.dart';
import 'package:floy/src/rust/api/init.dart';
import 'package:flutter/material.dart';
import 'app_screen.dart';

class InitScreen extends StatefulWidget {
  const InitScreen({super.key});

  @override
  _InitScreenState createState() => _InitScreenState();
}

class _InitScreenState extends State<InitScreen> {
  @override
  void initState() {
    super.initState();
    init();
  }

  Future<void> init() async {
    await initContext(root: await cross.root());
    Navigator.of(context)
        .pushReplacement(MaterialPageRoute(builder: (_) => const AppScreen()));
  }

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      body: Center(
        child: Text('Init Screen'),
      ),
    );
  }
}
