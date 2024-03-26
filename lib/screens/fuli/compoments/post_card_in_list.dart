import 'package:flutter/material.dart';
import 'package:flutter_html/flutter_html.dart';

import '../../../src/rust/client/fuli/dataobject.dart';

class PostCardInList extends StatelessWidget {
  final Post post;

  const PostCardInList({super.key, required this.post});

  @override
  Widget build(BuildContext context) {
    var content = SizedBox(
      child: Column(
        children: [
          Html(
            data: post.title.rendered,
            style: {
              "*": Style(
                fontSize: FontSize(18),
                padding: HtmlPaddings.zero,
                margin: Margins.zero,
              ),
            },
          ),
          Container(height: 8),
          SizedBox(
            height: 80,
            child: Row(
              children: [
                ...acfImage(),
                Expanded(
                  child: Html(
                    data: post.excerpt.rendered,
                    style: {
                      "*": Style(
                        fontSize: FontSize(14),
                        padding: HtmlPaddings.zero,
                        margin: Margins.zero,
                        color: Colors.grey,
                      ),
                    },
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
    var padded = Container(
      margin: const EdgeInsets.fromLTRB(10, 10, 10, 10),
      child: content,
    );
    var bordered = Container(
      decoration: BoxDecoration(
        border: Border(
          bottom: BorderSide(color: Colors.grey.shade300, width: .5),
        ),
      ),
      child: padded,
    );
    return padded;
  }

  List<Widget> acfImage() {
    if (post.acf.img == null) {
      return [];
    }
    return [
      ClipRRect(
        borderRadius: const BorderRadius.all(Radius.circular(2)),
        child: Image.network(
          post.acf.img!,
          fit: BoxFit.cover,
          width: 120,
          height: 80,
          errorBuilder: (context, error, stackTrace) {
            return Container(
              color: Colors.grey.shade300,
              width: 120,
              height: 80,
              child: Center(
                child: Icon(
                  Icons.error,
                  color: Colors.grey.shade500,
                ),
              ),
            );
          },
        ),
      ),
      Container(width: 8),
    ];
  }
}
