/**
 * @fileoverview
 * @enhanceable
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

goog.provide('proto.Catalyst.Protocol.Transaction.CFTransactionEntry');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.Catalyst.Protocol.Transaction.EntryRangeProof');


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
proto.Catalyst.Protocol.Transaction.CFTransactionEntry = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.Catalyst.Protocol.Transaction.CFTransactionEntry, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.Catalyst.Protocol.Transaction.CFTransactionEntry.displayName = 'proto.Catalyst.Protocol.Transaction.CFTransactionEntry';
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
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.toObject = function(opt_includeInstance) {
  return proto.Catalyst.Protocol.Transaction.CFTransactionEntry.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.Catalyst.Protocol.Transaction.CFTransactionEntry} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.toObject = function(includeInstance, msg) {
  var f, obj = {
    address: msg.getAddress_asB64(),
    pedersencommit: msg.getPedersencommit_asB64(),
    entryrangeproofs: (f = msg.getEntryrangeproofs()) && proto.Catalyst.Protocol.Transaction.EntryRangeProof.toObject(includeInstance, f)
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
 * @return {!proto.Catalyst.Protocol.Transaction.CFTransactionEntry}
 */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.Catalyst.Protocol.Transaction.CFTransactionEntry;
  return proto.Catalyst.Protocol.Transaction.CFTransactionEntry.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.Catalyst.Protocol.Transaction.CFTransactionEntry} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.Catalyst.Protocol.Transaction.CFTransactionEntry}
 */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setAddress(value);
      break;
    case 2:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setPedersencommit(value);
      break;
    case 3:
      var value = new proto.Catalyst.Protocol.Transaction.EntryRangeProof;
      reader.readMessage(value,proto.Catalyst.Protocol.Transaction.EntryRangeProof.deserializeBinaryFromReader);
      msg.setEntryrangeproofs(value);
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
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.Catalyst.Protocol.Transaction.CFTransactionEntry.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.Catalyst.Protocol.Transaction.CFTransactionEntry} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getAddress_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      1,
      f
    );
  }
  f = message.getPedersencommit_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      2,
      f
    );
  }
  f = message.getEntryrangeproofs();
  if (f != null) {
    writer.writeMessage(
      3,
      f,
      proto.Catalyst.Protocol.Transaction.EntryRangeProof.serializeBinaryToWriter
    );
  }
};


/**
 * optional bytes Address = 1;
 * @return {string}
 */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.getAddress = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * optional bytes Address = 1;
 * This is a type-conversion wrapper around `getAddress()`
 * @return {string}
 */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.getAddress_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getAddress()));
};


/**
 * optional bytes Address = 1;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getAddress()`
 * @return {!Uint8Array}
 */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.getAddress_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getAddress()));
};


/** @param {!(string|Uint8Array)} value */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.setAddress = function(value) {
  jspb.Message.setProto3BytesField(this, 1, value);
};


/**
 * optional bytes PedersenCommit = 2;
 * @return {string}
 */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.getPedersencommit = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * optional bytes PedersenCommit = 2;
 * This is a type-conversion wrapper around `getPedersencommit()`
 * @return {string}
 */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.getPedersencommit_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getPedersencommit()));
};


/**
 * optional bytes PedersenCommit = 2;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getPedersencommit()`
 * @return {!Uint8Array}
 */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.getPedersencommit_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getPedersencommit()));
};


/** @param {!(string|Uint8Array)} value */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.setPedersencommit = function(value) {
  jspb.Message.setProto3BytesField(this, 2, value);
};


/**
 * optional EntryRangeProof EntryRangeProofs = 3;
 * @return {?proto.Catalyst.Protocol.Transaction.EntryRangeProof}
 */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.getEntryrangeproofs = function() {
  return /** @type{?proto.Catalyst.Protocol.Transaction.EntryRangeProof} */ (
    jspb.Message.getWrapperField(this, proto.Catalyst.Protocol.Transaction.EntryRangeProof, 3));
};


/** @param {?proto.Catalyst.Protocol.Transaction.EntryRangeProof|undefined} value */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.setEntryrangeproofs = function(value) {
  jspb.Message.setWrapperField(this, 3, value);
};


proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.clearEntryrangeproofs = function() {
  this.setEntryrangeproofs(undefined);
};


/**
 * Returns whether this field is set.
 * @return {!boolean}
 */
proto.Catalyst.Protocol.Transaction.CFTransactionEntry.prototype.hasEntryrangeproofs = function() {
  return jspb.Message.getField(this, 3) != null;
};


