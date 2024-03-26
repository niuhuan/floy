import 'dart:developer';

import 'package:floy/screens/commons/refresh_customer.dart';
import 'package:flutter/material.dart';
import 'package:pull_to_refresh/pull_to_refresh.dart';

import 'content_blank.dart';
import 'content_error.dart';

class PullList<T> extends StatefulWidget {
  final Future<List<T>> Function({required int page}) fetcher;
  final Widget Function(
    BuildContext context,
    BoxConstraints constraints,
    T record,
  ) itemBuilder;

  const PullList({super.key, required this.fetcher, required this.itemBuilder});

  @override
  State<PullList> createState() => _PullListState<T>();
}

class _PullListState<T> extends State<PullList<T>>
    with AutomaticKeepAliveClientMixin {
  @override
  bool get wantKeepAlive => true;

  late List<T> _records = [];
  bool error = false;
  bool finish = false;
  bool loading = false;
  int size = 10;
  int page = 1;

  late RefreshController _refreshController;

  var errmsg = null;


  @override
  void initState() {
    _refreshController = RefreshController(initialRefresh: true);
    super.initState();
  }

  @override
  void dispose() {
    _refreshController.dispose();
    super.dispose();
  }

  List<Widget> _buildRecords(BoxConstraints constraints) {
    return _records
        .map((e) => widget.itemBuilder(context, constraints, e))
        .toList();
  }

  _onRefresh() async {
    try {
      error = false;
      loading = true;
      var rsp = await widget.fetcher(page: 1);
      _records = rsp;
      page = 2;
      _refreshController.resetNoData();
      _refreshController.refreshCompleted();
    } catch (e, s) {
      const start =
          "AnyhowException(ServerError: ServerError { errcode: -1, errmsg: \"";
      const end = "\" })";
      var msg = "$e";
      if (msg.startsWith(start) && msg.endsWith(end)) {
        errmsg = msg.substring(start.length, msg.length - end.length);
      } else {
        errmsg = null;
      }
      log("$e\n$s");
      error = true;
      _refreshController.refreshFailed();
    } finally {
      loading = false;
      setState(() {});
    }
  }

  _onLoad() async {
    try {
      error = false;
      loading = true;
      var rsp = await widget.fetcher(page: page);
      _records.addAll(rsp);
      page++;
      _refreshController.loadComplete();
      if (rsp.isEmpty) {
        _refreshController.loadNoData();
        finish = true;
      }
    } catch (e, s) {
      print("$e\n$s");
      error = true;
      _refreshController.loadFailed();
    } finally {
      loading = false;
      setState(() {});
    }
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return LayoutBuilder(
      builder: (BuildContext context, BoxConstraints constraints) {
        return SmartRefresher(
          controller: _refreshController,
          enablePullDown: true,
          enablePullUp: true,
          onRefresh: _onRefresh,
          onLoading: _onLoad,
          header: customerHeader(context),
          footer: customerFooter(context, _records.isNotEmpty),
          child: error && _records.isEmpty
              ? ListView(
                  children: [
                    SizedBox(
                      width: constraints.maxWidth,
                      height: constraints.maxHeight,
                      child: ContentError(
                        onRefresh: () {
                          _refreshController.requestRefresh();
                        },
                        errmsg: errmsg,
                      ),
                    ),
                  ],
                )
              : finish && _records.isEmpty
                  ? ListView(
                      children: [
                        SizedBox(
                          width: constraints.maxWidth,
                          height: constraints.maxHeight,
                          child: const ContentBlank(),
                        ),
                      ],
                    )
                  : ListView(children: _buildRecords(constraints)),
        );
      },
    );
  }
}
