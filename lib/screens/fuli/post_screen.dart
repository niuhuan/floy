import 'package:floy/screens/commons/future_loader.dart';
import 'package:floy/src/rust/api/fuli.dart';
import 'package:flutter/material.dart';
import 'package:flutter_html/flutter_html.dart';
import '../../src/rust/client/fuli/dataobject.dart';

class PostScreen extends StatefulWidget {
  final Post postSample;

  const PostScreen({super.key, required this.postSample});

  @override
  State<PostScreen> createState() => _PostScreenState();
}

class _PostScreenState extends State<PostScreen> {
  var _key = UniqueKey();
  late Future<Post> _future = fuliPost(postId: widget.postSample.id);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.white,
      appBar: AppBar(
        backgroundColor: Colors.white,
        title: _searchRound(),
        actions: [
          IconButton(onPressed: () {}, icon: const Icon(Icons.more_horiz)),
        ],
      ),
      body: FutureLoader<Post>(
        key: _key,
        future: _future,
        successBuilder: _buildSuccess,
        onRefresh: () {
          setState(() {
            _key = UniqueKey();
            _future = fuliPost(postId: widget.postSample.id);
          });
        },
      ),
    );
  }

  Widget _buildSuccess(BuildContext context, Post requireData) {
    return ListView(children: [
      Html(data: requireData.title.rendered),
      Html(
        data: requireData.content.rendered,
        extensions: [
          TagWrapExtension(
              tagsToWrap: {'img'},
              builder: (child) {
                return FittedBox(
                  child: child,
                );
              }),
        ],
        style: {
          "p": Style(
            fontSize: FontSize(15),
            lineHeight: const LineHeight(1.5),
            margin: Margins.only(
              left: 0,
              top: 10,
              right: 0,
              bottom: 10,
            ),
            // margin: EdgeInsets.fromLTRB(0, 10, 0, 10),
          ),
        },
      ),
      SafeArea(child: Container()),
    ]);
  }

  Widget _searchRound() {
    return Container(
      height: 40,
      decoration: BoxDecoration(
        color: Colors.grey.shade100,
        borderRadius: BorderRadius.circular(20),
      ),
      child: Row(
        children: [
          Container(width: 15),
          const Icon(Icons.search),
          Container(width: 10),
          Text(
            "搜你想看的",
            style: TextStyle(
              fontSize: 18,
              color: Colors.grey.shade700,
            ),
          ),
        ],
      ),
    );
  }
}
