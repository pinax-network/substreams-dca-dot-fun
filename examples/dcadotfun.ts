import { BigInt, BigDecimal } from "@graphprotocol/graph-ts";
import {
  CancelOrder as CancelOrderEvent,
  CreateOrder as CreateOrderEvent,
  FillOrder as FillOrderEvent,
  RoleAdminChanged as RoleAdminChangedEvent,
  RoleGranted as RoleGrantedEvent,
  RoleRevoked as RoleRevokedEvent,
  PauseCreate as PauseCreateEvent,
  PauseFill as PauseFillEvent,
  SetExecutionVarience as SetExecutionVarienceEvent,
  SetFeeCollector as SetFeeCollectorEvent,
  SetMaxFeedAgeFillOrder as SetMaxFeedAgeFillOrderEvent,
  SetMaxFeedAgeCreateOrder as SetMaxFeedAgeCreateOrderEvent,
  SetMaxScalingFactor as SetMaxScalingFactorEvent,
  SetMaxSlippage as SetMaxSlippageEvent,
  SetMinExecutionValue as SetMinExecutionValueEvent,
  SetMinOrderFrequencyInterval as SetMinOrderFrequencyIntervalEvent,
  SetMinSlippage as SetMinSlippageEvent,
  SetYieldSplit as SetYieldSplitEvent,
  SetProtocolFee as SetProtocolFeeEvent,
  SetTokenProps as SetTokenPropsEvent,
  SetTokenState as SetTokenStateEvent,
  SetAavePool as SetAavePoolEvent,
} from "../generated/dcadotfun/dcadotfun";
import {
  CancelOrder,
  CreateOrder,
  FillOrder,
  RoleAdminChanged,
  RoleGranted,
  RoleRevoked,
  PauseCreate,
  PauseFill,
  SetExecutionVarience,
  SetFeeCollector,
  SetMaxFeedAgeFillOrder,
  SetMaxFeedAgeCreateOrder,
  SetMaxScalingFactor,
  SetMaxSlippage,
  SetMinExecutionValue,
  SetMinOrderFrequencyInterval,
  SetMinSlippage,
  SetYieldSplit,
  SetProtocolFee,
  SetTokenProps,
  SetTokenState,
  SetAavePool,
  Token,
  Order,
  TokenList,
  User,
} from "../generated/schema";

export function handleCancelOrder(event: CancelOrderEvent): void {
  let entity = new CancelOrder(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.orderId = event.params.orderId;
  entity.vault = event.params.vault;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();

  let order = new Order(event.params.orderId.toString());
  order.repeats = BigInt.fromI32(0);
  order.canceled = true;
  order.save();
}

export function handleCreateOrder(event: CreateOrderEvent): void {
  let entity = new CreateOrder(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.orderId = event.params.orderId;
  entity.creator = event.params.creator;
  entity.recipient = event.params.recipient;
  entity.tokenIn = event.params.tokenIn;
  entity.tokenOut = event.params.tokenOut;
  entity.spendAmount = event.params.spendAmount;
  entity.repeats = event.params.repeats;
  entity.slippage = event.params.slippage;
  entity.freqInterval = event.params.freqInterval;
  entity.scalingInterval = event.params.scalingInterval;
  entity.lastRun = event.params.lastRun;
  entity.protocolFee = event.params.protocolFee;
  entity.vault = event.params.vault;
  entity.stakeAssetIn = event.params.stakeAssetIn;
  entity.stakeAssetOut = event.params.stakeAssetOut;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();

  let user = User.load(event.params.creator);

  if (!user) {
    user = new User(event.params.creator);
    user.points = BigDecimal.fromString("0");
    user.save();
  }

  let tokenIn = Token.load(event.params.tokenIn);
  let tokenOut = Token.load(event.params.tokenOut);

  if (!tokenIn || !tokenOut) {
    return;
  }

  let order = new Order(event.params.orderId.toString());
  order.creator = event.params.creator;
  order.recipient = event.params.recipient;
  order.tokenIn = tokenIn.id;
  order.tokenOut = tokenOut.id;
  order.spendAmount = event.params.spendAmount;
  order.repeats = event.params.repeats;
  order.slippage = event.params.slippage;
  order.freqInterval = event.params.freqInterval;
  order.scalingInterval = event.params.scalingInterval;
  order.lastRun = event.params.lastRun;
  order.nextRun = event.params.lastRun.plus(event.params.freqInterval);
  order.protocolFee = event.params.protocolFee;
  order.vault = event.params.vault;
  order.stakeAssetIn = event.params.stakeAssetIn;
  order.stakeAssetOut = event.params.stakeAssetOut;
  order.canceled = false;
  order.blockNumber = event.block.number;
  order.blockTimestamp = event.block.timestamp;
  order.transactionHash = event.transaction.hash;
  order.save();
}

export function handleFillOrder(event: FillOrderEvent): void {
  let entity = new FillOrder(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.orderId = event.params.orderId;
  entity.caller = event.params.caller;
  entity.recipient = event.params.recipient;
  entity.fillAmount = event.params.fillAmount;
  entity.amountOfTokenOut = event.params.amountOfTokenOut;
  entity.protocolFee = event.params.protocolFee;
  entity.tokenInPrice = event.params.tokenInPrice;
  entity.tokenOutPrice = event.params.tokenOutPrice;
  entity.scalingFactor = event.params.scalingFactor;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;
  entity.order = event.params.orderId.toString();

  entity.save();

  let order = Order.load(event.params.orderId.toString());
  if (order) {
    order.repeats = order.repeats.minus(BigInt.fromI32(1));
    order.lastRun = event.block.timestamp;
    order.nextRun = event.block.timestamp.plus(order.freqInterval);
    order.save();

    // update token metrics
    let tokenIn = Token.load(order.tokenIn);
    let tokenOut = Token.load(order.tokenOut);
    let tokenOutDecimals = BigInt.fromI32(0);

    if (tokenIn) {
      tokenIn.totalAmountFilled = tokenIn.totalAmountFilled.plus(
        event.params.fillAmount
      );
      tokenIn.save();
    }

    if (tokenOut) {
      tokenOut.totalAmountReceived = tokenOut.totalAmountReceived.plus(
        event.params.amountOfTokenOut
      );
      tokenOutDecimals = BigInt.fromI32(18).plus(tokenOut.tokenDecimals);

      tokenOut.save();
    }

    // award points to creator and filler
    let creator = User.load(order.creator);
    let filler = User.load(event.params.caller);

    let totalPoints = BigDecimal.fromString(event.params.protocolFee.toString())
      .times(BigDecimal.fromString(event.params.tokenOutPrice.toString()))
      .times(BigDecimal.fromString("100"))
      .div(
        BigDecimal.fromString(
          BigInt.fromI32(10)
            .pow(tokenOutDecimals.toI32() as u8)
            .toString()
        )
      );

    if (creator) {
      creator.points = creator.points.plus(totalPoints);
      creator.save();
    }

    if (!filler) {
      filler = new User(event.params.caller);
      filler.points = BigDecimal.fromString("0");
      filler.save();
    }

    if (filler) {
      filler.points = filler.points.plus(totalPoints);
      filler.save();
    }
  }
}

export function handlePauseCreate(event: PauseCreateEvent): void {
  let entity = new PauseCreate(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.isPaused = event.params.isPaused;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handlePauseFill(event: PauseFillEvent): void {
  let entity = new PauseFill(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.isPaused = event.params.isPaused;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleSetFeeCollector(event: SetFeeCollectorEvent): void {
  let entity = new SetFeeCollector(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.feeCollector = event.params.feeCollector;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleSetProtocolFee(event: SetProtocolFeeEvent): void {
  let entity = new SetProtocolFee(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.protocolFee = event.params.protocolFee;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleSetTokenState(event: SetTokenStateEvent): void {
  let entity = new SetTokenState(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.token = event.params.token;
  entity.isActive = event.params.isActive;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();

  let token = Token.load(event.params.token);
  if (token) {
    token.isActive = event.params.isActive;
    token.save();
  }
}

export function handleRoleAdminChanged(event: RoleAdminChangedEvent): void {
  let entity = new RoleAdminChanged(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.role = event.params.role;
  entity.previousAdminRole = event.params.previousAdminRole;
  entity.newAdminRole = event.params.newAdminRole;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleRoleGranted(event: RoleGrantedEvent): void {
  let entity = new RoleGranted(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.role = event.params.role;
  entity.account = event.params.account;
  entity.sender = event.params.sender;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleRoleRevoked(event: RoleRevokedEvent): void {
  let entity = new RoleRevoked(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.role = event.params.role;
  entity.account = event.params.account;
  entity.sender = event.params.sender;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleSetExecutionVarience(
  event: SetExecutionVarienceEvent
): void {
  let entity = new SetExecutionVarience(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.executionVarience = event.params.executionVarience;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleSetMaxFeedAgeFillOrder(
  event: SetMaxFeedAgeFillOrderEvent
): void {
  let entity = new SetMaxFeedAgeFillOrder(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.maxFeedAge = event.params.maxFeedAge;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleSetMaxFeedAgeCreateOrder(
  event: SetMaxFeedAgeCreateOrderEvent
): void {
  let entity = new SetMaxFeedAgeCreateOrder(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.maxFeedAge = event.params.maxFeedAge;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleSetMaxScalingFactor(
  event: SetMaxScalingFactorEvent
): void {
  let entity = new SetMaxScalingFactor(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.maxScalingFactor = event.params.maxScalingFactor;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleSetMaxSlippage(event: SetMaxSlippageEvent): void {
  let entity = new SetMaxSlippage(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.slippageMax = event.params.slippageMax;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleSetMinExecutionValue(
  event: SetMinExecutionValueEvent
): void {
  let entity = new SetMinExecutionValue(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.minExecutionValue = event.params.minExecutionValue;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleSetMinOrderFrequencyInterval(
  event: SetMinOrderFrequencyIntervalEvent
): void {
  let entity = new SetMinOrderFrequencyInterval(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.minOrderFrequencyInterval = event.params.minOrderFrequencyInterval;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleSetMinSlippage(event: SetMinSlippageEvent): void {
  let entity = new SetMinSlippage(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.slippageMin = event.params.slippageMin;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleSetYieldSplit(event: SetYieldSplitEvent): void {
  let entity = new SetYieldSplit(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.yieldSplit = event.params.yieldSplit;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}

export function handleSetTokenProps(event: SetTokenPropsEvent): void {
  let entity = new SetTokenProps(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.token = event.params.token;
  entity.feed = event.params.feed;
  entity.tokenDecimals = event.params.tokenDecimals;
  entity.tokenSymbol = event.params.tokenSymbol;
  entity.tokenName = event.params.tokenName;
  entity.isActive = event.params.isActive;
  entity.isStakable = event.params.isStakable;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();

  let token = Token.load(event.params.token);
  let tokenList = TokenList.load("all-tokens");

  if (!tokenList) {
    tokenList = new TokenList("all-tokens");
    tokenList.tokens = [];
    tokenList.save();
  }

  if (!token) {
    token = new Token(event.params.token);
    token.totalAmountFilled = BigInt.fromI32(0);
    token.totalAmountReceived = BigInt.fromI32(0);
    token.list = "all-tokens";

    let currentTokens = tokenList.tokens;
    currentTokens.push(token.id);
    tokenList.tokens = currentTokens;
    tokenList.save();
  }

  token.feed = event.params.feed;
  token.tokenDecimals = BigInt.fromI32(event.params.tokenDecimals);
  token.tokenSymbol = event.params.tokenSymbol;
  token.tokenName = event.params.tokenName;
  token.isActive = event.params.isActive;
  token.isStakable = event.params.isStakable;
  token.save();
}

export function handleSetAavePool(event: SetAavePoolEvent): void {
  let entity = new SetAavePool(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  );
  entity.aavePool = event.params.aavePool;

  entity.blockNumber = event.block.number;
  entity.blockTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}
