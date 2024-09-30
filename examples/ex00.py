def permutations(arr):
    # ベースケース: 配列が1つの要素しかない場合、そのまま返す
    if len(arr) == 1:
        return [arr]

    # 結果を格納するリスト
    result = []

    # 各要素に対して順列を生成
    for i in range(len(arr)):
        # 現在の要素を取り出し、残りの部分で再帰的に順列を生成
        current = arr[i]
        remaining = arr[:i] + arr[i + 1 :]

        # 残りの部分の順列を生成し、現在の要素と結合して結果に追加
        for p in permutations(remaining):
            result.append([current] + p)

    return result


# 使用例
nums = [0, 1, 2]
perm = permutations(nums)
for p in perm:
    print(p)
