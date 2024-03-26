import 'dart:async';

import 'package:floy/src/rust/client/fuli/dataobject.dart';
import 'package:flutter/material.dart';
import '../../../src/rust/api/fuli.dart';

class CategoryList extends StatefulWidget {
  final FutureOr<dynamic> Function({required Category? category}) onModify;

  const CategoryList({super.key, required this.onModify});

  @override
  State<CategoryList> createState() => _CategoryListState();
}

class _CategoryListState extends State<CategoryList> {
  var _feature = fuliCategories();
  var _key = UniqueKey();

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: _feature,
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return const Center(child: CircularProgressIndicator());
        }
        if (snapshot.hasError) {
          return const Center(child: Text('Error'));
        }
        return _buildSuccess(snapshot.requireData);
      },
    );
  }

  Widget _buildSuccess(List<Category> data) {
    return CategoryTab(data, onModify: widget.onModify);
  }
}

class CategoryTab extends StatefulWidget {
  final List<Category> categories;
  final FutureOr<dynamic> Function({required Category? category}) onModify;

  const CategoryTab(this.categories, {super.key, required this.onModify});

  @override
  State<CategoryTab> createState() => _CategoryTabState();
}

class _CategoryTabState extends State<CategoryTab>
    with SingleTickerProviderStateMixin {
  late final _tabController =
      TabController(length: widget.categories.length + 1, vsync: this);

  var currentIdx = 0;

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(
          child: TabBar(
            controller: _tabController,
            isScrollable: true,
            tabs: [
              const Tab(
                text: "最新",
              ),
              for (var item in widget.categories)
                Tab(
                  text: item.name,
                ),
            ],
            onTap: (idx) {
              currentIdx = idx;
              if (idx == 0) {
                widget.onModify(category: null);
              } else {
                widget.onModify(category: widget.categories[idx - 1]);
              }
            },
          ),
        ),
        Container(
          decoration: BoxDecoration(
            border: Border(
              bottom: BorderSide(
                color: Colors.grey.shade400,
                width: 1,
                style: BorderStyle.solid,
              ),
            ),
          ),
          child: IconButton(
            onPressed: () {
              showModalBottomSheet(
                context: context,
                builder: (BuildContext context) {
                  return CategoryBottomSheet(
                    categories: widget.categories,
                    onSelect: ({required Category? category}) {
                      if (category == null) {
                        currentIdx = 0;
                      } else {
                        currentIdx = widget.categories.indexOf(category) + 1;
                      }
                      _tabController.animateTo(currentIdx);
                      Navigator.of(context).pop();
                      widget.onModify(category: category);
                    },
                    selected: currentIdx == 0
                        ? null
                        : widget.categories[currentIdx - 1],
                  );
                },
                shape: const RoundedRectangleBorder(
                  borderRadius:
                      BorderRadius.vertical(top: Radius.circular(6.0)),
                ),
                backgroundColor: Colors.white,
                isDismissible: true,
              );
            },
            icon: const Icon(Icons.menu),
          ),
        ),
      ],
    );
  }
}

class CategoryBottomSheet extends StatefulWidget {
  final Category? selected;
  final List<Category> categories;
  final FutureOr<dynamic> Function({required Category? category}) onSelect;

  const CategoryBottomSheet({
    super.key,
    required this.categories,
    required this.onSelect,
    required this.selected,
  });

  @override
  State<CategoryBottomSheet> createState() => _CategoryBottomSheetState();
}

class _CategoryBottomSheetState extends State<CategoryBottomSheet> {
  @override
  Widget build(BuildContext context) {
    var size = MediaQuery.of(context).size;
    var itemWidth = (size.width - 20 - 40) / 4;
    return SizedBox(
      height: size.height * .8,
      child: Column(
        children: [
          Container(
            height: 50,
            decoration: BoxDecoration(
              border: Border(
                bottom: BorderSide(
                  width: .5,
                  color: Colors.grey.shade300,
                  style: BorderStyle.solid,
                ),
              ),
            ),
            child: Row(
              children: [
                IconButton(onPressed: () {}, icon: const Icon(Icons.close)),
                Expanded(child: Container()),
                IconButton(onPressed: () {}, icon: const Icon(Icons.search)),
              ],
            ),
          ),
          Expanded(
            child: ListView(
              children: [
                Padding(
                  padding: const EdgeInsets.all(10),
                  child: Wrap(
                    alignment: WrapAlignment.spaceBetween,
                    runAlignment: WrapAlignment.center,
                    runSpacing: 8,
                    children: [
                      GestureDetector(
                        onTap: () {
                          widget.onSelect(category: null);
                        },
                        child: Container(
                          width: itemWidth,
                          padding: const EdgeInsets.fromLTRB(8, 8, 8, 8),
                          decoration: BoxDecoration(
                            color: Colors.grey.shade100,
                          ),
                          child: Text(
                            "全部",
                            textAlign: TextAlign.center,
                            style: TextStyle(
                              color: widget.selected == null
                                  ? Colors.red
                                  : Colors.black,
                            ),
                          ),
                        ),
                      ),
                      for (var item in widget.categories)
                        GestureDetector(
                          onTap: () {
                            widget.onSelect(category: item);
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
                                color: item.id == widget.selected?.id
                                    ? Colors.red
                                    : Colors.black,
                              ),
                            ),
                          ),
                        ),
                    ],
                  ),
                ),
                Container(height: 30),
                SafeArea(child: Container()),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
