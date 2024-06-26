// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.28.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../client/fuli/dataobject.dart';
import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// The type `FoliCategoriesLoader` is not used by any `pub` functions, thus it is ignored.
// The type `FoliPostLoader` is not used by any `pub` functions, thus it is ignored.

Future<List<Category>> fuliCategories({dynamic hint}) =>
    RustLib.instance.api.fuliCategories(hint: hint);

Future<List<Post>> fuliPosts(
        {required int page, int? categoryId, dynamic hint}) =>
    RustLib.instance.api
        .fuliPosts(page: page, categoryId: categoryId, hint: hint);

Future<Post> fuliPost({required int postId, dynamic hint}) =>
    RustLib.instance.api.fuliPost(postId: postId, hint: hint);
