import 'package:floy/screens/commons/future_loader.dart';
import 'package:floy/src/rust/api/fuli.dart';
import 'package:flutter/material.dart';
import 'package:flutter_html/flutter_html.dart';
import '../../src/rust/client/community/dataobject.dart';
import '../../src/rust/api/bbs.dart';

class ThreadScreen extends StatefulWidget {
  final ThreadInList threadSample;

  const ThreadScreen({super.key, required this.threadSample});

  @override
  State<ThreadScreen> createState() => _ThreadScreenState();
}

class _ThreadScreenState extends State<ThreadScreen> {
  var _key = UniqueKey();
  late Future<ThreadData> _future = bbsThread(
    tid: widget.threadSample.fid,
    page: 1,
  );

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.white,
      appBar: AppBar(
        backgroundColor: Colors.white,
        actions: [
          IconButton(onPressed: () {}, icon: const Icon(Icons.more_horiz)),
        ],
      ),
    );
  }
}
