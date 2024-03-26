// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.28.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

import 'api/bbs.dart';
import 'api/fuli.dart';
import 'api/init.dart';
import 'api/user_center.dart';
import 'client/community/dataobject.dart';
import 'client/fuli/dataobject.dart';
import 'client/user_center/dataobject.dart';
import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw);

  @protected
  String dco_decode_String(dynamic raw);

  @protected
  Acf dco_decode_acf(dynamic raw);

  @protected
  BbsCategory dco_decode_bbs_category(dynamic raw);

  @protected
  bool dco_decode_bool(dynamic raw);

  @protected
  int dco_decode_box_autoadd_i_64(dynamic raw);

  @protected
  Category dco_decode_category(dynamic raw);

  @protected
  Content dco_decode_content(dynamic raw);

  @protected
  Forumlist dco_decode_forumlist(dynamic raw);

  @protected
  int dco_decode_i_64(dynamic raw);

  @protected
  Lastpost dco_decode_lastpost(dynamic raw);

  @protected
  List<BbsCategory> dco_decode_list_bbs_category(dynamic raw);

  @protected
  List<Category> dco_decode_list_category(dynamic raw);

  @protected
  List<Forumlist> dco_decode_list_forumlist(dynamic raw);

  @protected
  List<Medal> dco_decode_list_medal(dynamic raw);

  @protected
  List<Post> dco_decode_list_post(dynamic raw);

  @protected
  List<Postlist> dco_decode_list_postlist(dynamic raw);

  @protected
  Int64List dco_decode_list_prim_i_64_strict(dynamic raw);

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

  @protected
  List<Ratelog> dco_decode_list_ratelog(dynamic raw);

  @protected
  List<ThreadInList> dco_decode_list_thread_in_list(dynamic raw);

  @protected
  Login dco_decode_login(dynamic raw);

  @protected
  LoginState dco_decode_login_state(dynamic raw);

  @protected
  Medal dco_decode_medal(dynamic raw);

  @protected
  String? dco_decode_opt_String(dynamic raw);

  @protected
  int? dco_decode_opt_box_autoadd_i_64(dynamic raw);

  @protected
  Post dco_decode_post(dynamic raw);

  @protected
  Postlist dco_decode_postlist(dynamic raw);

  @protected
  Ratelog dco_decode_ratelog(dynamic raw);

  @protected
  ThreadData dco_decode_thread_data(dynamic raw);

  @protected
  ThreadDetail dco_decode_thread_detail(dynamic raw);

  @protected
  ThreadInList dco_decode_thread_in_list(dynamic raw);

  @protected
  ThreadPage dco_decode_thread_page(dynamic raw);

  @protected
  Title dco_decode_title(dynamic raw);

  @protected
  int dco_decode_u_8(dynamic raw);

  @protected
  void dco_decode_unit(dynamic raw);

  @protected
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer);

  @protected
  String sse_decode_String(SseDeserializer deserializer);

  @protected
  Acf sse_decode_acf(SseDeserializer deserializer);

  @protected
  BbsCategory sse_decode_bbs_category(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  int sse_decode_box_autoadd_i_64(SseDeserializer deserializer);

  @protected
  Category sse_decode_category(SseDeserializer deserializer);

  @protected
  Content sse_decode_content(SseDeserializer deserializer);

  @protected
  Forumlist sse_decode_forumlist(SseDeserializer deserializer);

  @protected
  int sse_decode_i_64(SseDeserializer deserializer);

  @protected
  Lastpost sse_decode_lastpost(SseDeserializer deserializer);

  @protected
  List<BbsCategory> sse_decode_list_bbs_category(SseDeserializer deserializer);

  @protected
  List<Category> sse_decode_list_category(SseDeserializer deserializer);

  @protected
  List<Forumlist> sse_decode_list_forumlist(SseDeserializer deserializer);

  @protected
  List<Medal> sse_decode_list_medal(SseDeserializer deserializer);

  @protected
  List<Post> sse_decode_list_post(SseDeserializer deserializer);

  @protected
  List<Postlist> sse_decode_list_postlist(SseDeserializer deserializer);

  @protected
  Int64List sse_decode_list_prim_i_64_strict(SseDeserializer deserializer);

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  List<Ratelog> sse_decode_list_ratelog(SseDeserializer deserializer);

  @protected
  List<ThreadInList> sse_decode_list_thread_in_list(
      SseDeserializer deserializer);

  @protected
  Login sse_decode_login(SseDeserializer deserializer);

  @protected
  LoginState sse_decode_login_state(SseDeserializer deserializer);

  @protected
  Medal sse_decode_medal(SseDeserializer deserializer);

  @protected
  String? sse_decode_opt_String(SseDeserializer deserializer);

  @protected
  int? sse_decode_opt_box_autoadd_i_64(SseDeserializer deserializer);

  @protected
  Post sse_decode_post(SseDeserializer deserializer);

  @protected
  Postlist sse_decode_postlist(SseDeserializer deserializer);

  @protected
  Ratelog sse_decode_ratelog(SseDeserializer deserializer);

  @protected
  ThreadData sse_decode_thread_data(SseDeserializer deserializer);

  @protected
  ThreadDetail sse_decode_thread_detail(SseDeserializer deserializer);

  @protected
  ThreadInList sse_decode_thread_in_list(SseDeserializer deserializer);

  @protected
  ThreadPage sse_decode_thread_page(SseDeserializer deserializer);

  @protected
  Title sse_decode_title(SseDeserializer deserializer);

  @protected
  int sse_decode_u_8(SseDeserializer deserializer);

  @protected
  void sse_decode_unit(SseDeserializer deserializer);

  @protected
  int sse_decode_i_32(SseDeserializer deserializer);

  @protected
  void sse_encode_AnyhowException(
      AnyhowException self, SseSerializer serializer);

  @protected
  void sse_encode_String(String self, SseSerializer serializer);

  @protected
  void sse_encode_acf(Acf self, SseSerializer serializer);

  @protected
  void sse_encode_bbs_category(BbsCategory self, SseSerializer serializer);

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_i_64(int self, SseSerializer serializer);

  @protected
  void sse_encode_category(Category self, SseSerializer serializer);

  @protected
  void sse_encode_content(Content self, SseSerializer serializer);

  @protected
  void sse_encode_forumlist(Forumlist self, SseSerializer serializer);

  @protected
  void sse_encode_i_64(int self, SseSerializer serializer);

  @protected
  void sse_encode_lastpost(Lastpost self, SseSerializer serializer);

  @protected
  void sse_encode_list_bbs_category(
      List<BbsCategory> self, SseSerializer serializer);

  @protected
  void sse_encode_list_category(List<Category> self, SseSerializer serializer);

  @protected
  void sse_encode_list_forumlist(
      List<Forumlist> self, SseSerializer serializer);

  @protected
  void sse_encode_list_medal(List<Medal> self, SseSerializer serializer);

  @protected
  void sse_encode_list_post(List<Post> self, SseSerializer serializer);

  @protected
  void sse_encode_list_postlist(List<Postlist> self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_i_64_strict(
      Int64List self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_strict(
      Uint8List self, SseSerializer serializer);

  @protected
  void sse_encode_list_ratelog(List<Ratelog> self, SseSerializer serializer);

  @protected
  void sse_encode_list_thread_in_list(
      List<ThreadInList> self, SseSerializer serializer);

  @protected
  void sse_encode_login(Login self, SseSerializer serializer);

  @protected
  void sse_encode_login_state(LoginState self, SseSerializer serializer);

  @protected
  void sse_encode_medal(Medal self, SseSerializer serializer);

  @protected
  void sse_encode_opt_String(String? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_i_64(int? self, SseSerializer serializer);

  @protected
  void sse_encode_post(Post self, SseSerializer serializer);

  @protected
  void sse_encode_postlist(Postlist self, SseSerializer serializer);

  @protected
  void sse_encode_ratelog(Ratelog self, SseSerializer serializer);

  @protected
  void sse_encode_thread_data(ThreadData self, SseSerializer serializer);

  @protected
  void sse_encode_thread_detail(ThreadDetail self, SseSerializer serializer);

  @protected
  void sse_encode_thread_in_list(ThreadInList self, SseSerializer serializer);

  @protected
  void sse_encode_thread_page(ThreadPage self, SseSerializer serializer);

  @protected
  void sse_encode_title(Title self, SseSerializer serializer);

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer);

  @protected
  void sse_encode_unit(void self, SseSerializer serializer);

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer);
}

// Section: wire_class

class RustLibWire implements BaseWire {
  factory RustLibWire.fromExternalLibrary(ExternalLibrary lib) =>
      RustLibWire(lib.ffiDynamicLibrary);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  RustLibWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;
}
