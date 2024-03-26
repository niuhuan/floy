import 'package:floy/src/rust/api/bbs.dart';
import 'package:floy/src/rust/client/community/dataobject.dart';
import 'package:flutter/material.dart';

import 'components/bbs_categories_list.dart';
import 'components/thread_list.dart';

class CommunityScreen extends StatefulWidget {
  const CommunityScreen({Key? key}) : super(key: key);

  @override
  State<CommunityScreen> createState() => _CommunityScreenState();
}

class _CommunityScreenState extends State<CommunityScreen> {
  String categoryId = "1";

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
            child: BBSCategoryList(
              onModify: (fid, name, description) async {
                setState(() {
                  categoryId = fid;
                });
              },
            ),
          ),
          Expanded(
            child: ThreadList(
              key: Key("THREAD_LIST:$categoryId"),
              fetcher: ({required int page}) async {
                return (await bbsThreadsList(page: page, fid: categoryId)).threadlist;
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
