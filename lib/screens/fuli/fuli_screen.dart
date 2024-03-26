import 'package:floy/screens/fuli/compoments/post_list.dart';
import 'package:floy/src/rust/api/fuli.dart';
import 'package:flutter/material.dart';
import '../../src/rust/client/fuli/dataobject.dart';
import 'compoments/category_list.dart';

class FuliScreen extends StatefulWidget {
  const FuliScreen({Key? key}) : super(key: key);

  @override
  State<FuliScreen> createState() => _FuliScreenState();
}

class _FuliScreenState extends State<FuliScreen> {
  int? categoryId;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: _searchRound(),
        backgroundColor: Colors.red.shade400,
      ),
      body: Column(
        children: [
          SizedBox(
            height: 40,
            child: CategoryList(
              onModify: ({required Category? category}) async {
                setState(() {
                  categoryId = category?.id;
                });
              },
            ),
          ),
          Expanded(
            child: PostList(
              key: Key("POST_LIST:$categoryId"),
              fetcher: ({required int page}) async {
                return fuliPosts(page: page, categoryId: categoryId);
              },
            ),
          ),
        ],
      ),
    );
  }

  Widget _searchRound() {
    return Container(
      height: 40,
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(20),
      ),
      child: Row(
        children: [
          Container(width: 15),
          const Icon(Icons.search),
          Container(width: 10),
          const Text(
            'App',
            style: TextStyle(color: Colors.black, fontSize: 18),
          ),
        ],
      ),
    );
  }
}
