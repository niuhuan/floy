import 'package:floy/screens/commons/pull_list.dart';
import 'package:floy/screens/fuli/compoments/post_card_in_list.dart';
import 'package:flutter/material.dart';
import '../../../src/rust/client/fuli/dataobject.dart';
import '../post_screen.dart';

class PostList extends PullList<Post> {
  PostList({super.key, required super.fetcher})
      : super(itemBuilder: (context, constraints, record) {
          return mapPostToCard(context, constraints, record);
        });
}

Widget mapPostToCard(
  BuildContext context,
  BoxConstraints constraints,
  Post post,
) {
  var postCardInList = PostCardInList(post: post);
  var taped = GestureDetector(
    onTap: () {
      Navigator.of(context).push(MaterialPageRoute(builder: (context) {
        return PostScreen(postSample: post);
      }));
    },
    child: postCardInList,
  );
  return taped;
}
