class TrieNode
  attr_accessor :children, :end_of_word

  def initialize
    @children = [nil] * 26
    @end_of_word = false
  end

  # Returns the idx of the child node in the @children's array
  def self.child_idx(letter)
    letter.ord - 'a'.ord
  end
end

class Trie
  def initialize
    @root = TrieNode.new
  end

  def add(word)
    raise ArgumentError, "can't add nil" if word.nil?
    raise ArgumentError, "must only add strings: #{word}" unless word.class == String
    raise ArgumentError, "string must consist only of lowercase letters: #{word}" if /[^a-z]/.match?(word)

    add_helper(@root, 0, word)
  end

  def search(pattern)
    search_helper(@root, 0, pattern)
  end

  private

  # Invariant: for add_helper and search_helper,
  # the curr_node parameter will always be non-nil

  def add_helper(curr_node, idx, word)
    if idx == word.length
      curr_node.end_of_word = true
    else
      # Does the current node contain the next letter?
      ci = TrieNode.child_idx(word[idx])
      next_node = curr_node.children[ci]
      if next_node.nil?
        # We aren't storing that letter here. Let's add it here,
        # and recurse down to finish adding the word
        next_node = TrieNode.new
        curr_node.children[ci] = next_node
      end
      add_helper(next_node, idx + 1, word)
    end
  end

  def search_helper(curr_node, idx, pattern)
    return curr_node.end_of_word if idx == pattern.length

    # Does the current node contain the next letter?

    # if the next letter is a wildcard (i.e. '.')
    # search all children underneath this node
    letter = pattern[idx]
    if is_wildcard?(letter)
      real_children = curr_node.children.reject(&:nil?)
      return false if real_children.empty?

      real_children
        .map { |child| search_helper(child, idx + 1, pattern) }
        .reduce { |a, b| a || b } # propagate true if any
    else
      # Traverse down normally
      ci = TrieNode.child_idx(pattern[idx])
      next_node = curr_node.children[ci]

      return false if next_node.nil?

      search_helper(next_node, idx + 1, pattern)
    end
  end

  # In this definition of Trie, a '.' denotes a wildcard character
  def is_wildcard?(letter)
    letter == '.'
  end
end
