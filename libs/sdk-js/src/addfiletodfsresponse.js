/**
 * @fileoverview
 * @enhanceable
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

goog.provide('proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse');

goog.require('jspb.Message');
goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');


/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.displayName = 'proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse';
}


if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse} msg The msg instance to transform.
 * @return {!Object}
 */
proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    responseCode: msg.getResponseCode_asB64(),
    dfsHash: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse}
 */
proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse;
  return proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse}
 */
proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setResponseCode(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setDfsHash(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse} message
 * @param {!jspb.BinaryWriter} writer
 */
proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResponseCode_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      1,
      f
    );
  }
  f = message.getDfsHash();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional bytes response_code = 1;
 * @return {string}
 */
proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.prototype.getResponseCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * optional bytes response_code = 1;
 * This is a type-conversion wrapper around `getResponseCode()`
 * @return {string}
 */
proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.prototype.getResponseCode_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getResponseCode()));
};


/**
 * optional bytes response_code = 1;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getResponseCode()`
 * @return {!Uint8Array}
 */
proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.prototype.getResponseCode_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getResponseCode()));
};


/** @param {!(string|Uint8Array)} value */
proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.prototype.setResponseCode = function(value) {
  jspb.Message.setField(this, 1, value);
};


/**
 * optional string dfs_hash = 2;
 * @return {string}
 */
proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.prototype.getDfsHash = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/** @param {string} value */
proto.Catalyst.Protocol.Rpc.Node.AddFileToDfsResponse.prototype.setDfsHash = function(value) {
  jspb.Message.setField(this, 2, value);
};


