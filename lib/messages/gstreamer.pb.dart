//
//  Generated code. Do not modify.
//  source: gstreamer.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class Gstreamer extends $pb.GeneratedMessage {
  factory Gstreamer({
    $core.String? verison,
  }) {
    final $result = create();
    if (verison != null) {
      $result.verison = verison;
    }
    return $result;
  }
  Gstreamer._() : super();
  factory Gstreamer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Gstreamer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Gstreamer', package: const $pb.PackageName(_omitMessageNames ? '' : 'gstreamer'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'verison')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Gstreamer clone() => Gstreamer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Gstreamer copyWith(void Function(Gstreamer) updates) => super.copyWith((message) => updates(message as Gstreamer)) as Gstreamer;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Gstreamer create() => Gstreamer._();
  Gstreamer createEmptyInstance() => create();
  static $pb.PbList<Gstreamer> createRepeated() => $pb.PbList<Gstreamer>();
  @$core.pragma('dart2js:noInline')
  static Gstreamer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Gstreamer>(create);
  static Gstreamer? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get verison => $_getSZ(0);
  @$pb.TagNumber(1)
  set verison($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasVerison() => $_has(0);
  @$pb.TagNumber(1)
  void clearVerison() => clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');

const ID = 1;