import 'dart:async';

import 'package:floy/screens/commons/content_error.dart';
import 'package:flutter/material.dart';

class FutureLoader<T> extends StatefulWidget {
  final Future<T> future;
  final Function() onRefresh;
  final Widget Function(BuildContext, T) successBuilder;

  const FutureLoader(
      {super.key,
      required this.future,
      required this.onRefresh,
      required this.successBuilder});

  @override
  State<FutureLoader> createState() => _FutureLoaderState<T>();
}

class _FutureLoaderState<T> extends State<FutureLoader<T>> {
  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: widget.future,
      builder: (BuildContext context, AsyncSnapshot<T> snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return const Center(child: CircularProgressIndicator());
        }
        if (snapshot.hasError) {
          return ContentError(onRefresh: widget.onRefresh);
        }
        return widget.successBuilder(context, snapshot.requireData);
      },
    );
  }
}
