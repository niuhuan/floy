import 'package:flutter/material.dart';
import '../../../src/rust/client/community/dataobject.dart';
import '../../../src/rust/api/bbs.dart';

class BBSCategoryList extends StatefulWidget {
  final Function(String, String, String) onModify;

  const BBSCategoryList({Key? key, required this.onModify}) : super(key: key);

  @override
  State<BBSCategoryList> createState() => _BBSCategoryListState();
}

class _BBSCategoryListState extends State<BBSCategoryList> {
  var fid = "1";
  var fname = "热门板块";

  Future<List<BbsCategory>> _future = bbsCategories();

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: _future,
      builder:
          (BuildContext context, AsyncSnapshot<List<BbsCategory>> snapshot) {
        if (snapshot.hasError) {
          return Row(
            children: [
              Expanded(child: Container()),
              IconButton(onPressed: () {}, icon: Icon(Icons.error))
            ],
          );
        }
        if (snapshot.connectionState != ConnectionState.done) {
          return Row(
            children: [
              Expanded(child: Container()),
              IconButton(onPressed: () {}, icon: Icon(Icons.downloading))
            ],
          );
        }
        return _buildSuccess(snapshot.requireData);
      },
    );
  }

  Widget _buildSuccess(List<BbsCategory> categories) {
    return Row(
      children: [
        Expanded(
          child: Container(
            padding: const EdgeInsets.fromLTRB(10, 0, 0, 0),
            child: Text(fname),
          ),
        ),
        IconButton(
          onPressed: () {
            showModalBottomSheet(
              context: context,
              builder: (BuildContext context) {
                return BbsCategoryBottomSheet(
                  categories: categories,
                  onSelect: (fid, name, description) {
                    this.fid = fid;
                    this.fname = name;
                    widget.onModify(
                      fid,
                      name,
                      description,
                    );
                    Navigator.of(context).pop();
                  },
                  selected: fid,
                );
              },
              shape: const RoundedRectangleBorder(
                borderRadius: BorderRadius.vertical(top: Radius.circular(6.0)),
              ),
              backgroundColor: Colors.white,
              isDismissible: true,
            );
          },
          icon: const Icon(Icons.menu),
        ),
      ],
    );
  }
}

class BbsCategoryBottomSheet extends StatefulWidget {
  final String selected;
  final List<BbsCategory> categories;
  final Function(String, String, String) onSelect;

  const BbsCategoryBottomSheet({
    super.key,
    required this.categories,
    required this.onSelect,
    required this.selected,
  });

  @override
  State<BbsCategoryBottomSheet> createState() => _BbsCategoryBottomSheetState();
}

class _BbsCategoryBottomSheetState extends State<BbsCategoryBottomSheet> {
  @override
  Widget build(BuildContext context) {
    var size = MediaQuery.of(context).size;
    var itemWidth = (size.width - 20 - 40) / 4;
    var spacing = 13.0;
    return ListView(
      children: [
        for (var pc in widget.categories) ...[
          GestureDetector(
            onTap: () {
              widget.onSelect(
                pc.fid,
                pc.name,
                "",
              );
            },
            child: Container(
              margin: const EdgeInsets.fromLTRB(0, 10, 0, 3),
              padding: const EdgeInsets.fromLTRB(8, 8, 8, 8),
              decoration: BoxDecoration(
                color: Colors.grey.shade100,
              ),
              child: Text(
                pc.name,
                textAlign: TextAlign.center,
                style: TextStyle(
                  color: pc.fid == widget.selected ? Colors.red : Colors.black,
                ),
              ),
            ),
          ),
          Wrap(
            spacing: spacing,
            runAlignment: WrapAlignment.center,
            runSpacing: 8,
            children: [
              for (var item in pc.forumlist)
                GestureDetector(
                  onTap: () {
                    widget.onSelect(
                      item.fid,
                      item.name,
                      item.description,
                    );
                  },
                  child: Container(
                    width: itemWidth,
                    padding: const EdgeInsets.fromLTRB(8, 8, 8, 8),
                    decoration: BoxDecoration(
                      color: Colors.grey.shade100,
                    ),
                    child: Text(
                      item.name,
                      textAlign: TextAlign.center,
                      style: TextStyle(
                        color: item.fid == widget.selected
                            ? Colors.red
                            : Colors.black,
                      ),
                    ),
                  ),
                ),
            ],
          ),
        ],
      ],
    );
  }
}
