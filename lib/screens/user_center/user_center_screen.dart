import 'package:flutter/material.dart';

class UserCenterScreen extends StatefulWidget {
  const UserCenterScreen({super.key});

  @override
  State<StatefulWidget> createState() => _UserCenterScreen();
}

class _UserCenterScreen extends State<UserCenterScreen> {
  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      body: Center(
        child: Text('User Center Screen'),
      ),
    );
  }
}
