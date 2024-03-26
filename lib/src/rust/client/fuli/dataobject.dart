// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.28.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

class Acf {
  final String? img;
  final String? content;

  const Acf({
    this.img,
    this.content,
  });

  @override
  int get hashCode => img.hashCode ^ content.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Acf &&
          runtimeType == other.runtimeType &&
          img == other.img &&
          content == other.content;
}

class Category {
  final int id;
  final int count;
  final String description;
  final String link;
  final String name;
  final String slug;
  final String taxonomy;
  final int parent;

  const Category({
    required this.id,
    required this.count,
    required this.description,
    required this.link,
    required this.name,
    required this.slug,
    required this.taxonomy,
    required this.parent,
  });

  @override
  int get hashCode =>
      id.hashCode ^
      count.hashCode ^
      description.hashCode ^
      link.hashCode ^
      name.hashCode ^
      slug.hashCode ^
      taxonomy.hashCode ^
      parent.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Category &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          count == other.count &&
          description == other.description &&
          link == other.link &&
          name == other.name &&
          slug == other.slug &&
          taxonomy == other.taxonomy &&
          parent == other.parent;
}

class Content {
  final String rendered;
  final bool protected;

  const Content({
    required this.rendered,
    required this.protected,
  });

  @override
  int get hashCode => rendered.hashCode ^ protected.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Content &&
          runtimeType == other.runtimeType &&
          rendered == other.rendered &&
          protected == other.protected;
}

class Post {
  final int id;
  final String date;
  final Title title;
  final Content excerpt;
  final Content content;
  final String commentStatus;
  final Int64List categories;
  final Int64List tags;
  final Acf acf;

  const Post({
    required this.id,
    required this.date,
    required this.title,
    required this.excerpt,
    required this.content,
    required this.commentStatus,
    required this.categories,
    required this.tags,
    required this.acf,
  });

  @override
  int get hashCode =>
      id.hashCode ^
      date.hashCode ^
      title.hashCode ^
      excerpt.hashCode ^
      content.hashCode ^
      commentStatus.hashCode ^
      categories.hashCode ^
      tags.hashCode ^
      acf.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Post &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          date == other.date &&
          title == other.title &&
          excerpt == other.excerpt &&
          content == other.content &&
          commentStatus == other.commentStatus &&
          categories == other.categories &&
          tags == other.tags &&
          acf == other.acf;
}

class Title {
  final String rendered;

  const Title({
    required this.rendered,
  });

  @override
  int get hashCode => rendered.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Title &&
          runtimeType == other.runtimeType &&
          rendered == other.rendered;
}