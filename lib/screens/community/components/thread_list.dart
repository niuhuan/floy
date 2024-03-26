import 'package:floy/screens/commons/pull_list.dart';
import 'package:floy/screens/community/components/thread_card_in_list.dart';
import 'package:flutter/material.dart';
import 'package:flutter_html/flutter_html.dart';
import '../../../src/rust/client/community/dataobject.dart';
import '../thread_screen.dart';

class ThreadList extends PullList<ThreadInList> {
  ThreadList({super.key, required super.fetcher})
      : super(itemBuilder: (context, constraints, record) {
          return mapThreadToCard(context, constraints, record);
        });
}

Widget mapThreadToCard(
  BuildContext context,
  BoxConstraints constraints,
  ThreadInList thread,
) {
  var threadCardInList = ThreadCardInList(thread: thread);
  var taped = GestureDetector(
    onTap: () {
      Navigator.of(context).push(MaterialPageRoute(builder: (context) {
        return ThreadScreen(threadSample: thread);
      }));
    },
    child: threadCardInList,
  );
  return taped;
}
