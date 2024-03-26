import 'package:flutter/material.dart';
import 'package:pull_to_refresh/pull_to_refresh.dart';

Widget customerHeader(BuildContext context) => CustomHeader(
      builder: (BuildContext context, RefreshStatus? mode) {
        late Widget widget;
        if (mode == RefreshStatus.refreshing) {
          widget = Text("正在接受福利数据");
        } else if (mode == RefreshStatus.completed) {
          widget = Text("正在接受福利数据");
        } else if (mode == RefreshStatus.failed) {
          widget = Text("未能成功连接福利");
        } else if (mode == RefreshStatus.idle) {
          widget = Text("下拉刷新");
        } else {
          widget = Text("下拉刷新");
        }
        return Center(
          child: widget,
        );
      },
    );

Widget customerFooter(BuildContext context, bool recordsIsEmpty) =>
    CustomFooter(
      builder: (BuildContext context, LoadStatus? mode) {
        Widget? widget;
        if (mode == LoadStatus.idle && recordsIsEmpty) {
          widget = Text("加载数据");
        } else if (mode == LoadStatus.canLoading && recordsIsEmpty) {
          widget = Text("加载数据");
        } else if (mode == LoadStatus.loading) {
          widget = Text("加载数据");
        }
        return widget == null
            ? Container()
            : Center(
                child: widget,
              );
      },
    );
