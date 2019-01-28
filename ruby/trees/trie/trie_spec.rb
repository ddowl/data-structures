# frozen_string_literal: true

require_relative './trie.rb'

describe Trie do
  let(:trie) { Trie.new }
  describe '#add' do
    it 'raises ArgumentError if passed nil' do
      expect { trie.add(nil) }.to raise_error(ArgumentError)
    end

    it 'raises ArgumentError if passed a non-string' do
      expect { trie.add(123) }.to raise_error(ArgumentError)
    end

    it 'raises ArgumentError if passes a string not in /[a-z]*/' do
      expect { trie.add('This string is INVALID!!!') }.to raise_error(ArgumentError)
    end

    it 'can add an empty string to the collection' do
      trie.add('')
    end

    it 'adds a string to the collection' do
      trie.add('hello')
    end
  end

  describe '#search' do
    context 'empty trie' do
      it 'is false for all strings' do
        expect(trie.search('potato')).to be(false)
      end
    end

    context 'trie contains one word' do
      before { trie.add('word') }

      it 'is true for only that word' do
        expect(trie.search('word')).to be(true)
        expect(trie.search('other')).to be(false)
      end

      it 'can match the word with wildcards' do
        expect(trie.search('w.rd')).to be(true)
        expect(trie.search('wo.d')).to be(true)
        expect(trie.search('....')).to be(true)
      end

      it 'fails if wildcard pattern is incorrect' do
        expect(trie.search('...')).to be(false)
        expect(trie.search('.....')).to be(false)
      end
    end
  end
end
