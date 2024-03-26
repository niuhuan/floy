import 'package:floy/src/rust/client/community/dataobject.dart';
import 'package:flutter/material.dart';

class ThreadCardInList extends StatelessWidget {
  final ThreadInList thread;

  const ThreadCardInList({super.key, required this.thread});

  @override
  Widget build(BuildContext context) {
    var content = SizedBox(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              ClipRRect(
                borderRadius: const BorderRadius.all(Radius.circular(300)),
                child: Image.network(
                  thread.avatar,
                  fit: BoxFit.cover,
                  width: 35,
                  height: 35,
                ),
              ),
              Container(width: 10),
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    thread.author,
                    style: const TextStyle(
                      fontSize: 16,
                    ),
                  ),
                  Container(width: 10),
                  Text(
                    thread.dateline,
                    style: const TextStyle(
                      fontSize: 14,
                      color: Colors.grey,
                    ),
                  ),
                ],
              ),
            ],
          ),
          Container(height: 5),
          Text(
            thread.subject,
            textAlign: TextAlign.left,
            style: const TextStyle(
              fontSize: 18,
            ),
          ),
          Container(height: 5),
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
    return bordered;
  }
}
