import 'package:flutter/material.dart';

class ContentError extends StatelessWidget {
  final Function() onRefresh;
  final String? errmsg;

  const ContentError({super.key, required this.onRefresh, this.errmsg});

  @override
  Widget build(BuildContext context) {
    return Center(
      // todo 一个空盒子的图片, onTap
      child: Text(errmsg ?? "ao, 出错了"),
    );
  }
}
