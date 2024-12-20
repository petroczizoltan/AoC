import 'package:shared/shared.dart';

Future<void> main(List<String> args) async {
  final List<int> leftLocationIds = [];
  final List<int> rightLocationIds = [];
  final Map<int, int> rightLocationIdsCounts = {};

  await Shared.readFile('input.txt', lineTransformer: (String line) {
    final List<String> splitLine = line.split(' ');

    final int leftLocationId = int.parse(splitLine.first);
    final int rightLocationId = int.parse(splitLine.last);

    leftLocationIds.add(leftLocationId);
    rightLocationIds.add(rightLocationId);
    rightLocationIdsCounts[rightLocationId] =
        (rightLocationIdsCounts[rightLocationId] ?? 0) + 1;
  });

  leftLocationIds.sort();
  rightLocationIds.sort();

  Iterator<int> leftIterator = leftLocationIds.iterator;
  Iterator<int> rightIterator = rightLocationIds.iterator;

  int distancesSum = 0;

  while (leftIterator.moveNext()) {
    rightIterator.moveNext();

    final int leftLocationId = leftIterator.current;
    final int rightLocationId = rightIterator.current;

    final int distance = (leftLocationId - rightLocationId).abs();
    distancesSum += distance;
  }

  print('1: $distancesSum');

  int similaritySum = 0;
  for (final int leftLocationId in leftLocationIds) {
    final int count = rightLocationIdsCounts[leftLocationId] ?? 0;
    similaritySum += leftLocationId * count;
  }

  print('2: $similaritySum');
}
